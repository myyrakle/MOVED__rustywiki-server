use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error,
};
use futures::future::{ok, Ready};
use futures::Future;

pub struct Auth;

impl Auth {
    pub fn new() -> Auth {
        Auth {}
    }
}

// 미들웨어 is `Transform` trait from actix-service crate
// `S` - 다음 서비스 타입
// `B` - 리스폰스 바디 타입
impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

use crate::lib::{jwt, AuthValue};

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    // wating
    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(context)
    }

    // call
    fn call(&mut self, request: ServiceRequest) -> Self::Future 
    {
        let (request, payload) = request.into_parts();

        let _path = request.path().to_string();
        let token = request.headers().get("authorization");

        println!("{:?}", request.headers());

        let mut auth_value = AuthValue::new();

        if token.is_some() {
            // 인증 처리
            // ...
            let token = token.unwrap().to_str().unwrap().to_string();
            println!("{}", token);

            let decoded_result = jwt::verify(token);

            if decoded_result.is_some() {
                use diesel::*;
                //use std::borrow::Borrow;
                use actix_web::web::Data;
                use std::sync::Mutex;

                let _f: &Data<Mutex<PgConnection>> = request.app_data().unwrap();

                auth_value.set_values(true, 1, "f".into());
                //Ok(decoded_result.unwrap().claims.data)
            }
        }

        request.extensions_mut().insert(auth_value);

        //let extensions = request.extensions();
        //let auth: &AuthValue = extensions.get::<AuthValue>().unwrap();

        let service_request = ServiceRequest::from_parts(request, payload).ok().unwrap();

        //let value = result.unwrap();

        let fut = self.service.call(service_request);

        Box::pin(async move {
            let response = fut.await?;
            Ok(response)
        })
    }
}
