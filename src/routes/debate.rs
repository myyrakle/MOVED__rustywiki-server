// standard
use std::borrow::Borrow;
use std::sync::Mutex;

// thirdparty
use actix_web::{
    get, http::StatusCode, post, web, web::Data, HttpRequest, HttpResponse, Responder,
};
use diesel::*;
use serde::{Deserialize, Serialize};

// in crate
use crate::lib::{make_offset, init_pagination. AuthValue};
use crate::models::{InsertDebate, InsertDebateComment};
use crate::response::{ServerErrorResponse, UnauthorizedResponse};
use crate::schema::{tb_debate, tb_debate_comment, tb_document, tb_user};
use crate::value::Debate;

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

#[derive(Deserialize, Serialize, Debug)]
pub struct GetDebateListParam {
    pub document_title: String,
    pub open_yn: Option<bool>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub next_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetDebateListResponse {
    pub success: bool,
    pub list: Vec<Debate>,
    pub total_count: i64,
    pub message: String,
}

#[get("/doc/debate-list")]
pub async fn get_debate_list(
    web::Query(query): web::Query<GetDebateListParam>,
    _request: HttpRequest,
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

    let result: Result<(Vec<Debate>, i64), diesel::result::Error> = connection.transaction(|| {
        //let next_token
        let (offset, limit) = init_pagination(query.page, query.limit, query.next_token);
        
        let document_id: i64 = tb_document::dsl::tb_document
            .filter(tb_document::dsl::title.eq(&query.document_title))
            .select(tb_document::dsl::id)
            .get_result(connection)?;

        use diesel::dsl::count_star;
        let list_and_count: (Vec<_>, i64) = if let Some(open_yn) = query.open_yn {
            let list = tb_debate::table
                .inner_join(tb_user::table.on(tb_user::dsl::id.eq(tb_debate::dsl::writer_id)))
                .filter(tb_debate::dsl::document_id.eq(document_id))
                .filter(tb_debate::dsl::open_yn.eq(open_yn))
                .order(tb_debate::dsl::reg_utc.desc())
                .offset(offset)
                .limit(limit)
                .select((
                    tb_debate::dsl::id,
                    tb_debate::dsl::writer_id,
                    tb_user::dsl::nickname,
                    tb_debate::dsl::subject,
                    tb_debate::dsl::content,
                    tb_debate::dsl::reg_utc,
                ))
                .get_results::<Debate>(connection)?;

            let total_count: i64 = tb_debate::dsl::tb_debate
                .filter(tb_debate::dsl::document_id.eq(document_id))
                .filter(tb_debate::dsl::open_yn.eq(open_yn))
                .select(count_star())
                .get_result(connection)?;

            (list, total_count)
        } else {
            let list = tb_debate::table
                .inner_join(tb_user::table.on(tb_user::dsl::id.eq(tb_debate::dsl::writer_id)))
                .filter(tb_debate::dsl::document_id.eq(document_id))
                .order(tb_debate::dsl::reg_utc.desc())
                .offset(offset)
                .limit(limit)
                .select((
                    tb_debate::dsl::id,
                    tb_debate::dsl::writer_id,
                    tb_user::dsl::nickname,
                    tb_debate::dsl::subject,
                    tb_debate::dsl::content,
                    tb_debate::dsl::reg_utc,
                ))
                .get_results::<Debate>(connection)?;

            let total_count: i64 = tb_debate::dsl::tb_debate
                .filter(tb_debate::dsl::document_id.eq(document_id))
                .select(count_star())
                .get_result(connection)?;

            (list, total_count)
        };

        Ok(list_and_count)
    });

    // 문서 존재 여부로 분기 처리
    match result {
        Ok((list, total_count)) => {
            let response = GetDebateListResponse {
                success: true,
                list: list,
                total_count: total_count,
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
