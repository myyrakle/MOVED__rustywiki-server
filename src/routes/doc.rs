// standard
use std::borrow::Borrow;
use std::sync::Mutex;
use std::fs;

// thirdparty
use actix_web::{
    get, http::StatusCode, post, web, web::Data, HttpRequest, HttpResponse, Responder,
};
use diesel::*;
use diesel::dsl::{exists, select};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// in crate
use crate::lib::{AuthValue, DOCUMENT_ROOT_PATH};
use crate::models::{SelectDocument, InsertDocument, InsertDocumentHistory, SelectDocumentHistory};
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

#[post("/doc/document")]
pub async fn write_doc(
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

    // 미인증 접근 거부
    let extensions = request.extensions();
    let auth: &AuthValue = extensions.get::<AuthValue>().unwrap();
    if !auth.is_authorized() {
        let response = UnauthorizedResponse::new();
        return HttpResponse::build(StatusCode::UNAUTHORIZED).json(response);
    }

    // 문서 존재여부 확인
    let exists_document_result = select(exists(tb_document::dsl::tb_document
        .filter(tb_document::dsl::title.eq(&body.title))))    
        .get_result(connection);

    // 문서 개수
    let content_length = body.content.chars().count();

    // 문서 텍스트 파일 생성
    let filename = Uuid::new_v4().to_string();
    let filepath = format!("{}/{}.txt", DOCUMENT_ROOT_PATH, filename);

    if fs::write(filepath, &body.content).is_err() {
        let response = ServerErrorResponse::new();
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
    }

    // 문서가 존재 여부로 분기 처리
    match exists_document_result {
        Ok(exists_document) => {
            if exists_document {
                // 문서 최근 수정일 변경 및
                // 문서 히스토리 추가

                let result = connection.transaction(||{
                    let document_id: i64 = diesel::update(tb_document::dsl::tb_document)
                        .filter(tb_document::dsl::title.eq(&body.title))
                        .set(tb_document::dsl::update_utc.eq(1))
                        .returning(tb_document::dsl::id)
                        .get_result(connection)?;

                    let document_history = InsertDocumentHistory {
                        writer_id: auth.user_id, 
                        document_id: document_id, 
                        filepath: "".into(), 
                        increase: content_length as i64, 
                    };

                    diesel::insert_into(tb_document_history::table)
                        .values(document_history)
                        .execute(connection)
                });

                let response = WriteDocResponse {
                    success:true, 
                    is_new_doc: false,
                    message: "문서 작성 성공".into()
                };
                HttpResponse::build(StatusCode::OK).json(response)
            } else {
                // 문서 최초 생성
                let document = InsertDocument{
                    title: body.title.clone()
                };

                let result = connection.transaction(||{
                    let document_id: i64 = diesel::insert_into(tb_document::table)
                        .values(document)
                        .returning(tb_document::dsl::id)
                        .get_result(connection)?;

                    let document_history = InsertDocumentHistory {
                        writer_id: auth.user_id, 
                        document_id: document_id, 
                        filepath: filepath, 
                        increase: content_length as i64, 
                    };

                    diesel::insert_into(tb_document_history::table)
                        .values(document_history)
                        .execute(connection)
                });

                match result { 
                    Ok(_) => {
                        let response = WriteDocResponse {
                            success: true, 
                            is_new_doc: true,
                            message: "문서 작성 성공".into()
                        };
                        HttpResponse::build(StatusCode::OK).json(response)
                    }, 
                    Err(_) => {
                        let response = ServerErrorResponse::new();
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response)
                    }
                }
            }
        }, 
        Err(_)=> {
            let response = ServerErrorResponse::new();
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response)
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReadDocParam {
    pub title: String,
}

#[get("/doc/document")]
pub async fn read_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}
