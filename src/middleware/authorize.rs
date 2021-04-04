use std::borrow::Borrow;
use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::Cookie,
    Error,
};
use futures::future::{ok, Ready};
use futures::Future;

use crate::models::SelectUser;
use crate::schema::tb_user;

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
    fn call(&mut self, request: ServiceRequest) -> Self::Future {
        let (request, payload) = request.into_parts();

        let _path = request.path().to_string();
        //let token = request.headers().get("access_token");
        let token = request
            .headers()
            .get("Cookie")
            .map(|cookie| cookie.to_str().ok())
            .flatten()
            .map(|cookie_text| Cookie::from_str(cookie_text).ok())
            .flatten()
            .map(|cookie| {
                println!("테스트");
                println!("쿠키 {:?}", cookie);
                println!("밸루 {:?}", cookie.value());
                cookie.value().to_owned()
            });

        //let cookie: Cookie = Cookie::from_str(cookie_text).unwrap();

        println!("{:?}", request.headers());

        let mut auth_value = AuthValue::new();

        if let Some(token) = token {
            let token = token.to_string();

            println!("{}", token);

            let decoded_result = jwt::verify(token);

            if let Some(decoded_result) = decoded_result {
                use diesel::*;
                //use std::borrow::Borrow;
                use actix_web::web::Data;
                use std::sync::Mutex;

                //let connection: &Data<Mutex<PgConnection>> = request.app_data();

                if let Some(connection) = request.app_data::<Data<Mutex<PgConnection>>>() {
                    if let Ok(connection) = connection.lock() {
                        let connection: &PgConnection = Borrow::borrow(&connection);

                        let user = tb_user::dsl::tb_user
                            .filter(tb_user::dsl::id.eq(decoded_result))
                            .get_result::<SelectUser>(connection);

                        if let Ok(user) = user {
                            auth_value.set_values(true, user.id, user.user_type);
                        }
                    }
                }
            }
        }

        //println!("{:?}", auth_value);
        request.extensions_mut().insert(auth_value);

        //let extensions = request.extensions();
        //let auth: &AuthValue = extensions.get::<AuthValue>().unwrap();

        let service_request = ServiceRequest::from_parts(request, payload).ok().unwrap();

        let fut = self.service.call(service_request);

        Box::pin(async move {
            let response = fut.await?;
            Ok(response)
        })
    }
}
