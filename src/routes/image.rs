// standard
use std::sync::Mutex;
use std::borrow::Borrow;

// thirdparty
use actix_multipart::Multipart;
use actix_web::{
    http::StatusCode, post, HttpRequest, web::Data, HttpResponse, Responder, 
};
use serde::{Deserialize, Serialize};
use diesel::*;
use diesel::dsl::{select, exists};

// in crate
use super::super::lib;
use super::super::models::InsertUser;
use super::super::schema::tb_user;
use super::super::response::{ServerErrorResponse, UnauthorizedResponse};
use lib::{AuthValue};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ImageUploadResponse {
    pub success: bool, 
    pub image_id: i64,
    pub image_url: String,
    pub image_too_big: bool,
}

#[post("/image/upload")]
pub async fn image_upload(mut payload: Multipart, request: HttpRequest, connection: Data<Mutex<PgConnection>>) -> impl Responder {
    let connection = match connection.lock() {
        Err(_) => {
            let response = ServerErrorResponse::new();
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
        }, 
        Ok(connection) => connection,
    };
    let connection:&PgConnection = Borrow::borrow(&connection);

    let extensions = request.extensions();
    let auth: &AuthValue = match extensions.get::<AuthValue>() {
        None => {
            let response = ServerErrorResponse::new();
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
        },
        Some(auth) => auth,
    };

    if auth.is_authorized() {
        let response = UnauthorizedResponse::new();
        return HttpResponse::build(StatusCode::UNAUTHORIZED).json(response);
    }

    let response = ImageUploadResponse{
        success: true, 
        image_id: 0, 
        image_url: "".to_owned(),
        image_too_big: false,
    };
    HttpResponse::build(StatusCode::OK).json(response)
}
