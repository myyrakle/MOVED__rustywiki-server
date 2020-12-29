use actix_web::{
    delete, get, http::StatusCode, post, put, web, web::Data, HttpRequest, HttpResponse, HttpMessage, Responder,
};
use serde::{Deserialize, Serialize};

use super::super::lib::jwt;

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupParam {
    pub email: String, 
    pub password: String, 
    pub nickname: String,
}

//web::Json(body): web::Json<SignupParam>

#[post("/signup")]
pub async fn signup(bytes: web::Bytes /*body:web::Json<SignupParam>*/) -> impl Responder {
    println!("{:?}", bytes.to_vec());
    let st = String::from_utf8(bytes.to_vec()).unwrap();
    //println!("{}", st);
    HttpResponse::build(StatusCode::OK).json(st)
    //foo.into_inner().into_string();
}

#[get("/token")]
pub async fn get_token() -> impl Responder {
    let token = jwt::sign(2, "foo".into());
    HttpResponse::build(StatusCode::OK).json(token)
    //foo.into_inner().into_string();
}
