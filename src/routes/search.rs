// standard
use std::borrow::Borrow;
use std::sync::Mutex;

// thirdparty
use actix_web::{
    get, http::StatusCode, post, web, web::Data, HttpRequest, HttpResponse, Responder,
};
use diesel::dsl::{exists, select};
use diesel::*;
use serde::{Deserialize, Serialize};

// in crate
use crate::lib::AuthValue;
use crate::models::{InsertDocument, InsertDocumentHistory, SelectDocument, SelectDocumentHistory};
use crate::response::{ServerErrorResponse, UnauthorizedResponse};
use crate::schema::{tb_document, tb_document_history};

#[derive(Deserialize, Serialize, Debug)]
pub struct WriteDocParam {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WriteDocResponse {
    pub success: bool,
    pub is_new_doc: bool,
    pub message: String,
}

#[post("/doc/search")]
pub async fn search(
    web::Json(body): web::Json<WriteDocParam>,
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
    let connection: &PgConnection = Borrow::borrow(&connection);

    let response = ServerErrorResponse::new();
    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json;
}
