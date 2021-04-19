// standard
use std::borrow::Borrow;
use std::sync::Mutex;

// thirdparty
use actix_web::{
    get, http::StatusCode, post, put, web, web::Data, HttpRequest, HttpResponse, Responder,
};
use diesel::dsl::{exists, select};
use diesel::*;
use serde::{Deserialize, Serialize};

// in crate
use crate::lib::AuthValue;
use crate::models::{InsertDebate, SelectDocument};
use crate::response::{ServerErrorResponse, UnauthorizedResponse};
use crate::schema::{tb_debate, tb_document};

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
        println!("ㅅ");
        let document_id: i64 = tb_document::dsl::tb_document
            .filter(tb_document::dsl::title.eq(&body.document_title))
            .select(tb_document::dsl::id)
            .get_result(connection)?;

        println!("됨1");

        let debate = InsertDebate {
            document_id: document_id,
            writer_id: auth.user_id,
            subject: body.subject,
            content: body.content,
        };

        println!("됨2");

        let debate_id: i64 = diesel::insert_into(tb_debate::table)
            .values(debate)
            .returning(tb_debate::dsl::id)
            .get_result(connection)?;

        println!("됨3");

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
