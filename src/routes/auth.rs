use actix_web::{
    delete, get, http::StatusCode, post, put, web, web::Data, HttpRequest, HttpResponse, HttpMessage, Responder,
};


use super::super::lib::jwt;

use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Debug)]
pub struct SignupParam {
    pub email: String, 
    pub password: String, 
    pub nickname: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupResponse {
    pub success: bool, 
}

//web::Json(body): web::Json<SignupParam>

#[post("/signup")]
pub async fn signup(web::Json(body): web::Json<SignupParam>) -> impl Responder {
    //println!("{:?}", bytes.to_vec());
    //let st = String::from_utf8(bytes.to_vec()).unwrap();
    //println!("{}", st);
    HttpResponse::build(StatusCode::OK).json(body)
}

#[get("/token")]
pub async fn get_token() -> impl Responder {
    let token = jwt::sign(2, "foo".into());
    HttpResponse::build(StatusCode::OK).json(token)
    //foo.into_inner().into_string();
}
