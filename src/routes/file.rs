// standard
use std::borrow::Borrow;
use std::io::Write;
use std::sync::Mutex;

// thirdparty
use actix_multipart::Multipart;
use actix_web::{http::StatusCode, post, web, web::Data, HttpRequest, HttpResponse, Responder};
use diesel::*;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
//use diesel::dsl::{select, exists};
use uuid::Uuid;

// in crate
use crate::lib;
//use crate::models::InsertUser;
//use crate::schema::tb_user;
use crate::response::{ServerErrorResponse, UnauthorizedResponse};
use lib::AuthValue;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ImageUploadResponse {
    pub success: bool,
    pub image_id: i64,
    pub image_url: String,
    pub image_too_big: bool,
}

#[post("/image")]
pub async fn upload_file(
    mut payload: Multipart,
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

    let extensions = request.extensions();
    let auth: &AuthValue = match extensions.get::<AuthValue>() {
        None => {
            let response = ServerErrorResponse::new();
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
        }
        Some(auth) => auth,
    };

    if auth.is_authorized() {
        let response = UnauthorizedResponse::new();
        return HttpResponse::build(StatusCode::UNAUTHORIZED).json(response);
    }

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().expect("1");

        let file_format = content_type
            .get_filename()
            .expect("foo")
            .split(".")
            .last()
            .unwrap_or("jpg");

        let filename = Uuid::new_v4().to_string();
        let filepath = format!("./static/image/{}.{}", filename, file_format);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .expect("3");

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .expect("4");
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
