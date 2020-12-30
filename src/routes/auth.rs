
use std::sync::Mutex;
use std::borrow::Borrow;

use actix_web::{
    http::StatusCode, post, web, web::Data, HttpResponse, Responder,
};

use super::super::lib;

use serde::{Deserialize, Serialize};

use diesel::*;
use diesel::dsl::{select, exists};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupParam {
    pub email: String, 
    pub password: String, 
    pub nickname: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupResponse {
    pub success: bool, 
    pub email_duplicated: bool,
    pub error: String,
}

//web::Json(body): web::Json<SignupParam>

use super::super::models::InsertUser;
use super::super::schema::tb_user;

#[post("/auth/signup")]
pub async fn signup(web::Json(body): web::Json<SignupParam>, connection: Data<Mutex<PgConnection>>) -> impl Responder {
    let connection = connection.lock().unwrap();	
    let connection:&PgConnection = Borrow::borrow(&connection);

    // email duplicated check
    //use tb_user::dsl::*;
    let already_exists = select(exists(
        tb_user::dsl::tb_user.filter(tb_user::dsl::email.eq_all(body.email.clone())))
    ).get_result(connection).unwrap();

    if already_exists { 
        let response = SignupResponse{
            success:false, 
            email_duplicated: true, 
            error: "email already exists".to_owned()
        };
        return HttpResponse::build(StatusCode::OK).json(response);
    } 

    // do signup
    let insert_value = InsertUser::new(body.email, body.password, body.nickname);

    diesel::insert_into(tb_user::table)
        .values(insert_value)
        .execute(connection)
        .unwrap();

    let response = SignupResponse{
        success: true, 
        email_duplicated: false, 
        error: "".to_owned()
    };
    HttpResponse::build(StatusCode::OK).json(response)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginParam {
    pub email: String, 
    pub password: String, 
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginResponse {
    pub success: bool, 
    pub login_failed: bool,
    pub token: String,
}

use super::super::models::SelectUser;

#[post("/auth/login")]
pub async fn login(web::Json(body): web::Json<LoginParam>, connection: Data<Mutex<PgConnection>>) -> impl Responder {
    let connection = connection.lock().unwrap();	
    let connection:&PgConnection = Borrow::borrow(&connection);

    let LoginParam{email, password} = body;
    let password = lib::hash(password); 

    //use tb_user::{dsl, dsl::{tb_user}};
    let foo = tb_user::dsl::tb_user
        .filter(tb_user::dsl::email.eq(&email))
        .filter(tb_user::dsl::email.eq(&password))
        .filter(tb_user::dsl::use_yn.eq(true))
        .load::<SelectUser>(connection)
        .unwrap();
    let token = lib::jwt::sign(2, "foo".into());
    HttpResponse::build(StatusCode::OK).json(token)
    //foo.into_inner().into_string();
}
