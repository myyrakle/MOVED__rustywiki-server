// standard
use std::sync::Mutex;
use std::borrow::Borrow;
use std::io::Write;

// thirdparty
use actix_multipart::{Multipart};
use futures::{StreamExt, TryStreamExt};
use actix_web::{
    web, http::StatusCode, post, HttpRequest, web::Data, HttpResponse, Responder, 
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

#[post("/image")]
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

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp");

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
        }
    }

    let response = ImageUploadResponse {
        success: true, 
        image_id: 0, 
        image_url: "".to_owned(), 
        image_too_big: false, 
    };
    HttpResponse::build(StatusCode::OK).json(response)
}
