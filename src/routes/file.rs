// standard
use std::borrow::Borrow;
use std::io::Write;
use std::sync::Mutex;

// thirdparty
use actix_multipart::Multipart;
use actix_web::{http::StatusCode, post, web, web::{Data}, HttpRequest, HttpResponse, Responder};
use diesel::*;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
//use diesel::dsl::{select, exists};
use uuid::Uuid;

// in crate
use crate::lib;
//use crate::models::InsertUser;
//use crate::schema::tb_user;
use crate::response::{ServerErrorResponse, UnauthorizedResponse, BadParameter};
use lib::AuthValue;

#[derive(Deserialize, Serialize, Debug)]
pub struct FileInfo {
    pub path: String,
    pub data: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileUploadParam {
    pub title: String,
    pub source: String,
    pub file: FileInfo,
}

impl FileUploadParam {
    pub async fn from(mut payload: Multipart) -> Option<FileUploadParam> {
        let mut title: Option<String> = None;
        let mut source: Option<String> = None;
        let mut file_data: Option<Vec<u8>> = None;
        let mut file_path: Option<String> = None;

        while let Ok(Some(mut field)) = payload.try_next().await {
            println!("{:?}", field);
    
            let e = field.content_disposition()?;
            let name = e.get_name()?;

            let mut bytes:Vec<web::Bytes> = Vec::new();

            while let Some(chunk) = field.next().await {
                bytes.push(chunk.unwrap());
            }

            let bytes: Vec<u8> = bytes.iter().map(|e| e.to_vec()).flatten().collect();

            match name {
                "title" => {
                    title = Some(String::from_utf8(bytes).ok()?);
                    println!("타이틀: {:?}", title);
                },
                "source" => {
                    source = Some(String::from_utf8(bytes).ok()?);
                    println!("출처: {:?}", source);
                },
                "file" => {
                    let file_format = e
                        .get_filename()
                        .expect("foo")
                        .split(".")
                        .last()
                        .unwrap_or("jpg");
                    let filename = Uuid::new_v4().to_string();

                    file_path = Some(format!("./static/image/{}.{}", filename, file_format));
                    file_data = Some(bytes);

                    println!("파일path: {:?}", file_path);
                },
                _ => {}
            }
        }

        Some(FileUploadParam {
            title: title?, 
            file: FileInfo {
                path: file_path?, 
                data: file_data?,
            },
            source: source?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileUploadResponse {
    pub success: bool,
    pub file_write_failed: bool,
    pub file_too_big: bool,
    pub image_id: i64,
    pub image_url: String,
}

#[post("/file")]
pub async fn upload_file(
    payload: Multipart,
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

    let body = FileUploadParam::from(payload).await;
    if body.is_none() {
        let response = BadParameter::new();
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
    }
    let body = body.unwrap();
    let file_path = body.file.path.clone();
    let file_data = body.file.data.clone();

    // 스레드풀을 사용한 파일 쓰기
    let file_write = async {
        let mut f = match web::block(move || std::fs::File::create(&file_path)).await { 
            Ok(f) => f, 
            Err(_) => {
                return None;
            }
        };
        match web::block(move || f.write_all(&file_data).map(|_| f)).await {
            Ok(_) => Some(()), 
            Err(_) =>  None,
        }
    }.await;
    
    if file_write.is_none() {
        let response = FileUploadResponse {
            success: false,
            file_write_failed: true,
            file_too_big: false,
            image_id: 0,
            image_url: "".to_owned(),
        };
        return HttpResponse::build(StatusCode::OK).json(response);
    }

    let response = FileUploadResponse {
        success: true,
        file_write_failed: false,
        image_id: 0,
        image_url: "".to_owned(),
        file_too_big: false,
    };
    HttpResponse::build(StatusCode::OK).json(response)
}
