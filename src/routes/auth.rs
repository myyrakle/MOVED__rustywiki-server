use actix_web::{
    delete, get, http::StatusCode, post, put, web, web::Data, HttpRequest, HttpResponse, Responder,
};

#[path = "../lib/jwt.rs"]
mod jwt;

#[get("/token")]
pub async fn get_token() -> impl Responder {
    let token = jwt::sign(2, "foo".into());
    HttpResponse::build(StatusCode::OK).json(token)
    //foo.into_inner().into_string();
}
