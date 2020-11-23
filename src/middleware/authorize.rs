use std::pin::Pin;
use std::task::{Context, Poll};

use std::rc;

use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse, Extensions}, Error, HttpRequest, FromRequest, web::Payload};
use futures::future::{ok, Ready};
use futures::Future;

pub struct Auth;

impl Auth 
{
    pub fn new() -> Auth 
    {
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

    fn new_transform(&self, service: S) -> Self::Future 
    {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> 
{
    service: S,
}

pub struct AuthInfo
{
    authorized: bool,
    user_id: i64,
    user_type: String, 
}

impl AuthInfo 
{
    pub fn is_authorized(&self) -> bool {
        self.authorized
    }
}

impl AuthInfo 
{
    pub fn new()-> AuthInfo
    {
        AuthInfo
        {
            authorized:false, 
            user_id:-1,
            user_type:"NO".into()
        }
    }
}

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
    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> 
    {
        self.service.poll_ready(context)
    }

    // call
    fn call(&mut self, mut request: ServiceRequest) -> Self::Future 
    {
        let path = request.path().to_string();
        //let token = request.headers().get("AUTHORIZAION").unwrap();

        use diesel::*;
        //use std::borrow::Borrow;
        use std::sync::{Mutex};
        use actix_web::{web::Data};

        let f:&Data<Mutex<PgConnection>> = request.app_data().unwrap();

        let mut auth = AuthInfo::new();
    
        let mut extensions = Extensions::new();
        extensions.insert(auth);
        request.add_data_container(rc::Rc::new(extensions));
        //let (httpRequest, _) = &request.into_parts(); 
        // httpRequest.extensions_mut().insert(auth);
        //httpRequest.extensions_mut().insert(auth);

        let fut = self.service.call(request);

        Box::pin(async move {
            let response = fut.await?;
            Ok(response)
        })
    }
}

// struct Authorized;

// impl FromRequest for Authorized {
//     type Error = ();
//     type Future = Result<(), Error>;
//     type Config = ();

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         if is_authorized(req) {
//             Ok(())
//         } else {
//             Err(())?
//         }
//     }
// }

// fn is_authorized(req: &HttpRequest) -> bool {
//     if let Some(value) = req.headers().get("authorized") {
//         // actual implementation that checks header here
//         dbg!(value);
//         true
//     } else {
//         false
//     }
// }