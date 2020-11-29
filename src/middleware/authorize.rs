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

pub struct AuthInfo {
    authorized: bool,
    user_id: i64,
    user_type: String,
}

impl AuthInfo {
    pub fn is_authorized(&self) -> bool {
        self.authorized
    }
}

impl AuthInfo {
    pub fn new() -> AuthInfo {
        AuthInfo {
            authorized: false,
            user_id: -1,
            user_type: "NO".into(),
        }
    }
}

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
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
    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(context)
    }

    // call
    fn call(&mut self, request: ServiceRequest) -> Self::Future {
        let (request, _payload) = request.into_parts();

        let _path = request.path().to_string();
        let token = request.headers().get("AUTHORIZAION");

        let mut auth = AuthInfo::new();

        let key = b"foo";

        if token.is_some() {
            // 인증 처리
            // ...
            let token = token.unwrap().to_str().unwrap();

            let decoded_result = jsonwebtoken::decode::<Claims>(
                token,
                &DecodingKey::from_secret(key),
                &Validation::new(Algorithm::HS256),
            );

            if decoded_result.is_ok() {
                use diesel::*;
                //use std::borrow::Borrow;
                use actix_web::web::Data;
                use std::sync::Mutex;

                let _f: &Data<Mutex<PgConnection>> = request.app_data().unwrap();

                auth.authorized = true;
                //Ok(decoded_result.unwrap().claims.data)
            }
        }

        request.extensions_mut().insert(auth);

        let service_request = ServiceRequest::from_request(request).unwrap();

        let fut = self.service.call(service_request);

        Box::pin(async move {
            let response = fut.await?;
            Ok(response)
        })
    }
}
