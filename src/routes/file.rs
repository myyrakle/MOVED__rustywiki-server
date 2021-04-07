// standard
use std::borrow::Borrow;
use std::io::Write;
use std::sync::Mutex;

// thirdparty
use actix_multipart::Multipart;
use actix_web::{
    get, http::StatusCode, post, put, web, web::Data, HttpRequest, HttpResponse, Responder,
};
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

    // 미인증 접근 거부
    let extensions = request.extensions();
    let nonauth = AuthValue::new();
    let auth: &AuthValue = extensions.get::<AuthValue>().unwrap_or(&nonauth);
    if !auth.is_authorized() {
        let response = UnauthorizedResponse::new();
        return HttpResponse::build(StatusCode::UNAUTHORIZED).json(response);
    }

    let body = FileUploadParam::from(payload).await;
    if body.is_none() {
        let response = BadParameter::new();
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
    }

    match body {
        Some(body) => {
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

                let history_id: i64 = diesel::insert_into(tb_file_history::table)
                    .values(insert_file_history)
                    .returning(tb_file_history::dsl::id)
                    .get_result(connection)?;

                diesel::update(tb_file::table)
                    .filter(tb_file::dsl::id.eq(file_id))
                    .set(tb_file::dsl::recent_history_id.eq(history_id))
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
        None => {
            let response = BadParameter::new();
            HttpResponse::build(StatusCode::BAD_REQUEST).json(response)
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileUpdateParam {
    pub title: String,
    pub content: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileUpdateResponse {
    pub success: bool,
    pub message: String,
}

#[put("/file")]
pub async fn update_file(
    web::Json(body): web::Json<FileUpdateParam>,
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

    // 미인증 접근 거부
    let extensions = request.extensions();
    let nonauth = AuthValue::new();
    let auth: &AuthValue = extensions.get::<AuthValue>().unwrap_or(&nonauth);
    if !auth.is_authorized() {
        let response = UnauthorizedResponse::new();
        return HttpResponse::build(StatusCode::UNAUTHORIZED).json(response);
    }

    let content_length = body
        .content
        .clone()
        .map(|e| e.chars().count() as i64)
        .unwrap_or(0);

    let result = connection.transaction::<_, Error, _>(|| {
        let file_id: i64 = tb_file::dsl::tb_file
            .filter(tb_file::dsl::title.eq(&body.title))
            .select(tb_file::dsl::id)
            .get_result(connection)?;

        let prev_count: i64 = tb_file_history::dsl::tb_file_history
            .filter(tb_file_history::dsl::file_id.eq(file_id))
            .filter(tb_file_history::dsl::latest_yn.eq(true))
            .select(tb_file_history::dsl::char_count)
            .get_result(connection)?;

        diesel::update(tb_file_history::dsl::tb_file_history)
            .filter(tb_file_history::dsl::file_id.eq(file_id))
            .filter(tb_file_history::dsl::latest_yn.eq(true))
            .set(tb_file_history::dsl::latest_yn.eq(false))
            .execute(connection)?;

        let insert_file_history = InsertFileHistory {
            file_id: file_id,
            writer_id: auth.user_id,
            content: body.content,
            char_count: content_length,
            increase: content_length - prev_count,
        };

        let history_id: i64 = diesel::insert_into(tb_file_history::table)
            .values(insert_file_history)
            .returning(tb_file_history::dsl::id)
            .get_result(connection)?;

        diesel::update(tb_file::table)
            .filter(tb_file::dsl::id.eq(file_id))
            .set(tb_file::dsl::recent_history_id.eq(history_id))
            .execute(connection)
    });

    match result {
        Ok(_) => {
            let response = FileUpdateResponse {
                success: true,
                message: "성공".into(),
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

#[derive(Deserialize, Serialize, Debug)]
pub struct FileReadParam {
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileReadResponse {
    pub success: bool,
    pub filepath: String,
    pub content: String,
    pub message: String,
}

#[get("/file")]
pub async fn read_file(
    web::Query(query): web::Query<FileUpdateParam>,
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

    let result: Result<(SelectFile, SelectFileHistory), diesel::result::Error> = (|| {
        let file: SelectFile = tb_file::dsl::tb_file
            .filter(tb_file::dsl::title.eq(&query.title))
            .get_result(connection)?;

        let history: SelectFileHistory = tb_file_history::dsl::tb_file_history
            .filter(tb_file_history::dsl::id.eq(file.recent_history_id.unwrap_or(-1)))
            .filter(tb_file_history::dsl::latest_yn.eq(true))
            .get_result(connection)?;

        Ok((file, history))
    })();

    match result {
        Ok((file, history)) => {
            let response = FileReadResponse {
                success: true,
                filepath: file.filepath,
                content: history.content,
                message: "성공".into(),
            };
            HttpResponse::build(StatusCode::OK).json(response)
        }
        Err(error) => {
            log::error!("query error: {}", error);
            let response = FileReadResponse {
                success: false,
                filepath: "".into(),
                content: "".into(),
                message: "실패".into(),
            };
            HttpResponse::build(StatusCode::OK).json(response)
        }
    }
}
