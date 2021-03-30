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
use diesel::result::Error;
use uuid::Uuid;

// in crate
use crate::lib;
//use crate::models::InsertUser;
use crate::models::{InsertFile, InsertFileHistory, SelectFile, SelectFileHistory};
use crate::response::{BadParameter, ServerErrorResponse, UnauthorizedResponse};
use crate::schema::{tb_file, tb_file_history};
use lib::AuthValue;

#[derive(Deserialize, Serialize, Debug)]
pub struct FileInfo {
    pub path: String,
    pub data: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileUploadParam {
    pub title: String,
    pub file: FileInfo,
    pub content: Option<String>,
}

// multipart 읽어 Parameter 타입으로 가공
impl FileUploadParam {
    pub async fn from(mut payload: Multipart) -> Option<FileUploadParam> {
        let mut title: Option<String> = None;
        let mut content: Option<String> = None;
        let mut file_data: Option<Vec<u8>> = None;
        let mut file_path: Option<String> = None;

        while let Ok(Some(mut field)) = payload.try_next().await {
            println!("{:?}", field);
            let e = field.content_disposition()?;
            let name = e.get_name()?;

            let mut bytes: Vec<web::Bytes> = Vec::new();

            while let Some(chunk) = field.next().await {
                if let Ok(chunk) = chunk {
                    bytes.push(chunk);
                }
            }

            let bytes: Vec<u8> = bytes.iter().map(|e| e.to_vec()).flatten().collect();

            match name {
                "title" => {
                    title = Some(String::from_utf8(bytes).ok()?);
                    println!("타이틀: {:?}", title);
                }
                "content" => {
                    content = Some(String::from_utf8(bytes).ok()?);
                    println!("내용: {:?}", content);
                }
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
                }
                _ => {}
            }
        }

        Some(FileUploadParam {
            title: title?,
            file: FileInfo {
                path: file_path?,
                data: file_data?,
            },
            content: content,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileUploadResponse {
    pub success: bool,
    pub file_write_failed: bool,
    pub file_too_big: bool,
    pub title_duplicate: bool,
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
    let connection: &PgConnection = Borrow::borrow(&connection);

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
            Err(_) => None,
        }
    }
    .await;
    if file_write.is_none() {
        let response = FileUploadResponse {
            success: false,
            file_write_failed: true,
            file_too_big: false,
            title_duplicate: false,
        };
        return HttpResponse::build(StatusCode::OK).json(response);
    }

    let result = connection.transaction::<_, Error, _>(|| {
        let insert_file = InsertFile {
            uploader_id: auth.user_id,
            title: body.title,
            filepath: body.file.path,
        };

        let file_id: i64 = diesel::insert_into(tb_file::table)
            .values(insert_file)
            .returning(tb_file::dsl::id)
            .get_result(connection)?;

        let insert_file_history = InsertFileHistory {
            file_id: file_id,
            writer_id: auth.user_id,
            content: body.content,
            char_count: 0,
            increase: 0,
        };

        diesel::insert_into(tb_file_history::table)
            .values(insert_file_history)
            .execute(connection)
    });

    match result {
        Ok(_) => {
            let response = FileUploadResponse {
                success: true,
                file_write_failed: false,
                file_too_big: false,
                title_duplicate: false,
            };
            HttpResponse::build(StatusCode::OK).json(response)
        }
        Err(error) => {
            log::error!("error: {}", error);
            let response = ServerErrorResponse::new();
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileUpdateResponse {
    pub success: bool,
}

#[post("/file")]
pub async fn update_file(
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
    let connection: &PgConnection = Borrow::borrow(&connection);

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
            Err(_) => None,
        }
    }
    .await;
    if file_write.is_none() {
        let response = FileUploadResponse {
            success: false,
            file_write_failed: true,
            file_too_big: false,
            title_duplicate: false,
        };
        return HttpResponse::build(StatusCode::OK).json(response);
    }

    let result = connection.transaction::<_, Error, _>(|| {
        let insert_file = InsertFile {
            uploader_id: auth.user_id,
            title: body.title,
            filepath: body.file.path,
        };

        let file_id: i64 = diesel::insert_into(tb_file::table)
            .values(insert_file)
            .returning(tb_file::dsl::id)
            .get_result(connection)?;

        let insert_file_history = InsertFileHistory {
            file_id: file_id,
            writer_id: auth.user_id,
            content: body.content,
            char_count: 0,
            increase: 0,
        };

        diesel::insert_into(tb_file_history::table)
            .values(insert_file_history)
            .execute(connection)
    });

    match result {
        Ok(_) => {
            let response = FileUploadResponse {
                success: true,
                file_write_failed: false,
                file_too_big: false,
                title_duplicate: false,
            };
            HttpResponse::build(StatusCode::OK).json(response)
        }
        Err(error) => {
            log::error!("error: {}", error);
            let response = ServerErrorResponse::new();
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response)
        }
    }
}
