// standard
use std::borrow::Borrow;
use std::sync::Mutex;

// thirdparty
use actix_web::{
    get, http::StatusCode, post, put, web, web::Data, HttpRequest, HttpResponse, Responder,
};
use diesel::*;
use serde::{Deserialize, Serialize};

// in crate
use crate::lib::AuthValue;
use crate::models::{InsertDebate, InsertDebateComment, SelectDocument};
use crate::response::{ServerErrorResponse, UnauthorizedResponse};
use crate::schema::{tb_debate, tb_debate_comment, tb_document};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDebateParam {
    pub document_title: String,
    pub subject: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateDebateResponse {
    pub success: bool,
    pub debate_id: i64,
    pub message: String,
}

#[post("/doc/debate")]
pub async fn create_debate(
    web::Json(body): web::Json<CreateDebateParam>,
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

    let debate_id: Result<i64, diesel::result::Error> = connection.transaction(|| {
        let document_id: i64 = tb_document::dsl::tb_document
            .filter(tb_document::dsl::title.eq(&body.document_title))
            .select(tb_document::dsl::id)
            .get_result(connection)?;

        let debate = InsertDebate {
            document_id: document_id,
            writer_id: auth.user_id,
            subject: body.subject,
            content: body.content,
        };

        let debate_id: i64 = diesel::insert_into(tb_debate::table)
            .values(debate)
            .returning(tb_debate::dsl::id)
            .get_result(connection)?;

        Ok(debate_id)
    });

    // 문서 존재 여부로 분기 처리
    match debate_id {
        Ok(debate_id) => {
            let response = CreateDebateResponse {
                success: true,
                debate_id: debate_id,
                message: "토론 등록 성공".into(),
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
pub struct WriteCommentParam {
    pub debate_id: i64,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WriteCommentResponse {
    pub success: bool,
    pub is_close_debate: bool,
    pub message: String,
}

#[post("/doc/debate/comment")]
pub async fn write_comment(
    web::Json(body): web::Json<WriteCommentParam>,
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

    let open_yn: Result<bool, diesel::result::Error> = connection.transaction(|| {
        let open_yn: bool = tb_debate::dsl::tb_debate
            .filter(tb_debate::dsl::id.eq(body.debate_id))
            .select(tb_debate::dsl::open_yn)
            .get_result(connection)?;

        if open_yn {
            let insert_comment = InsertDebateComment {
                debate_id: body.debate_id,
                writer_id: auth.user_id,
                content: body.content,
            };

            diesel::insert_into(tb_debate_comment::table)
                .values(insert_comment)
                .execute(connection)?;
        }

        Ok(open_yn)
    });

    // 문서 존재 여부로 분기 처리
    match open_yn {
        Ok(open_yn) => {
            if open_yn {
                let response = WriteCommentResponse {
                    success: true,
                    is_close_debate: false,
                    message: "토론 코멘트 등록 성공".into(),
                };
                HttpResponse::build(StatusCode::OK).json(response)
            } else {
                let response = WriteCommentResponse {
                    success: false,
                    is_close_debate: true,
                    message: "토론 코멘트 등록 성공".into(),
                };
                HttpResponse::build(StatusCode::OK).json(response)
            }
        }
        Err(error) => {
            log::error!("error: {}", error);
            let response = ServerErrorResponse::new();
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response)
        }
    }
}

// #[get("/doc/debate/open-list")]
// pub async fn get_debate_open_list(
//     web::Json(body): web::Json<CreateDebateParam>,
//     request: HttpRequest,
//     connection: Data<Mutex<PgConnection>>,
// ) -> impl Responder {
//     let connection = match connection.lock() {
//         Err(_) => {
//             log::error!("database connection lock error");
//             let response = ServerErrorResponse::new();
//             return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
//         }
//         Ok(connection) => connection,
//     };
//     let connection: &PgConnection = Borrow::borrow(&connection);

//     let debate_id: Result<i64, diesel::result::Error> = connection.transaction(|| {
//         let document_id: i64 = tb_document::dsl::tb_document
//             .filter(tb_document::dsl::title.eq(&body.document_title))
//             .select(tb_document::dsl::id)
//             .get_result(connection)?;

//         Ok(debate_id)
//     });

//     // 문서 존재 여부로 분기 처리
//     match debate_id {
//         Ok(debate_id) => {
//             let response = CreateDebateResponse {
//                 success: true,
//                 debate_id: debate_id,
//                 message: "토론 등록 성공".into(),
//             };
//             HttpResponse::build(StatusCode::OK).json(response)
//         }
//         Err(error) => {
//             log::error!("error: {}", error);
//             let response = ServerErrorResponse::new();
//             HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response)
//         }
//     }
// }
