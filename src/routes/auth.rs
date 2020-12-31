// standard
use std::sync::Mutex;
use std::borrow::Borrow;

// thirdparty
use actix_web::{
    http::StatusCode, post, web, web::Data, HttpResponse, Responder, 
};
use serde::{Deserialize, Serialize};
use diesel::*;
use diesel::dsl::{select, exists};

// in crate
use super::super::lib;
use super::super::models::InsertUser;
use super::super::schema::tb_user;
use super::super::response::{ServerErrorResponse};

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
    pub message: String,
}


#[post("/auth/signup")]
pub async fn signup(web::Json(body): web::Json<SignupParam>, connection: Data<Mutex<PgConnection>>) -> impl Responder {
    let connection = match connection.lock() {
        Err(_) => {
            let response = ServerErrorResponse::new();
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
        }, 
        Ok(connection) => connection,
    };
    let connection:&PgConnection = Borrow::borrow(&connection);

    // email duplicated check
    let already_exists = select(exists(
        tb_user::dsl::tb_user.filter(tb_user::dsl::email.eq(body.email.clone())))
    ).get_result(connection).unwrap();

    if already_exists { 
        let response = SignupResponse{
            success:false, 
            email_duplicated: true, 
            message: "email already exists".to_owned()
        };
        return HttpResponse::build(StatusCode::OK).json(response);
    } 

    use uuid::Uuid;
    let salt = Uuid::new_v4().to_string();

    // do signup
    let insert_value = InsertUser::new(body.email, body.password + &salt, salt, body.nickname);

    let execute_result = diesel::insert_into(tb_user::table)
        .values(insert_value)
        .execute(connection);

    if execute_result.is_err() {
        let response = ServerErrorResponse::new();
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
    }

    let response = SignupResponse{
        success: true, 
        email_duplicated: false, 
        message: "".to_owned()
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
    pub message: String,
}

use super::super::models::SelectUser;

#[post("/auth/login")]
pub async fn login(web::Json(body): web::Json<LoginParam>, connection: Data<Mutex<PgConnection>>) -> impl Responder {
    let connection = match connection.lock() {
        Err(_) => {
            let response = ServerErrorResponse::new();
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
        }, 
        Ok(connection) => connection,
    };
    let connection:&PgConnection = Borrow::borrow(&connection);

    let LoginParam{email, password} = body;

    let query = 
        tb_user::dsl::tb_user
        .filter(tb_user::dsl::email.eq(&email))
        .filter(tb_user::dsl::use_yn.eq(true));

    let user_result = 
        query.load::<SelectUser>(connection);

    match user_result {
        Ok(users) => {
            let response = if users.is_empty() {
                LoginResponse {
                    success: false, 
                    login_failed: true, 
                    token: "".to_owned(), 
                    message: "login failed".to_owned(),
                }
            } else {
                let user = &users[0];
                let salt = &user.salt;

                let password = lib::hash(password + salt);
                
                if password == user.password {
                    let token = lib::jwt::sign(user.id, user.user_type.clone());
                    LoginResponse {
                        success: true, 
                        login_failed: false, 
                        token: token, 
                        message: "".to_owned(),
                    }
                }
                else {
                    LoginResponse {
                        success: false, 
                        login_failed: true, 
                        token: "".to_owned(), 
                        message: "login failed".to_owned(),
                    }
                }
            };
            
            HttpResponse::build(StatusCode::OK).json(response)
        }
        Err(error) => {
            let response = LoginResponse {
                success: false, 
                login_failed: false, 
                token: "".to_owned(), 
                message: error.to_string(),
            };

            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response)
        }
    }
}
