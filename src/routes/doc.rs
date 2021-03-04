// standard
use std::borrow::Borrow;
use std::sync::Mutex;

// thirdparty
use actix_web::{
    delete, get, http::StatusCode, post, put, web::Data, HttpRequest, HttpResponse, Responder,
};
use diesel::*;
use serde::{Deserialize, Serialize};

// in crate
use crate::lib::AuthValue;
//use crate::models::SelectUser;
use crate::response::{ServerErrorResponse, UnauthorizedResponse};
//use crate::schema::tb_user;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDocParam {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateDocResponse {
    pub success: bool,
    pub email_duplicated: bool,
    pub message: String,
}

#[post("/doc/document")]
pub async fn create_doc(
    request: HttpRequest,
    connection: Data<Mutex<PgConnection>>,
) -> impl Responder {
    let connection = match connection.lock() {
        Err(_) => {
            log::error!("database connection lock error");
            let response = ServerErrorResponse::new();
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
        }
        Ok(connection) => connection,
    };
    let _connection: &PgConnection = Borrow::borrow(&connection);

    // 미인증 접근 거부
    let extensions = request.extensions();
    let auth: &AuthValue = extensions.get::<AuthValue>().unwrap();
    if !auth.is_authorized() {
        let response = UnauthorizedResponse::new();
        return HttpResponse::build(StatusCode::UNAUTHORIZED).json(response);
    }

    let response = UnauthorizedResponse::new();
    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
}

#[get("/doc/document")]
pub async fn read_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}

#[put("/doc/document")]
pub async fn update_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}

#[delete("/doc/document")]
pub async fn delete_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}
