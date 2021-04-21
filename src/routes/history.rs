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
use crate::lib::AuthValue;
use crate::models::{InsertDocumentHistory, SelectDocument, SelectDocumentHistory};
use crate::response::{ServerErrorResponse, UnauthorizedResponse};
use crate::schema::{tb_document, tb_document_history, tb_user};
use crate::value::DocumentHistory;

#[derive(Deserialize, Serialize, Debug)]
pub struct ReadHistoryParam {
    pub title: String,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReadHistoryResponse {
    pub success: bool,
    pub list: Vec<DocumentHistory>,
    pub total_count: i64,
    pub message: String,
}

#[get("/doc/history-list")]
pub async fn read_document_history_list(
    web::Query(query): web::Query<ReadHistoryParam>,
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

    let limit = query.limit.unwrap_or(10);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let result: Result<(Vec<DocumentHistory>, i64), diesel::result::Error> = (|| {
        let document: SelectDocument = tb_document::dsl::tb_document
            .filter(tb_document::dsl::title.eq(&query.title))
            .get_result(connection)?;

        let history_list: Vec<_> = tb_document_history::table
            .inner_join(tb_user::table.on(tb_user::dsl::id.eq(tb_document_history::dsl::writer_id)))
            .filter(tb_document_history::dsl::document_id.eq(document.id))
            .order(tb_document_history::dsl::reg_utc.desc())
            .offset(offset)
            .limit(limit)
            .select((
                tb_document_history::dsl::id,
                tb_document_history::dsl::content,
                tb_document_history::dsl::char_count,
                tb_document_history::dsl::increase,
                tb_document_history::dsl::reg_utc,
                tb_document_history::dsl::revision_number,
                tb_document_history::dsl::writer_id,
                tb_user::dsl::nickname,
            ))
            .get_results::<DocumentHistory>(connection)?;

        use diesel::dsl::count_star;
        let total_count: i64 = tb_document_history::dsl::tb_document_history
            .filter(tb_document_history::dsl::document_id.eq(document.id))
            .select(count_star())
            .get_result(connection)?;

        Ok((history_list, total_count))
    })();

    match result {
        Ok((history_list, total_count)) => {
            let response = ReadHistoryResponse {
                success: true,
                list: history_list,
                message: "".into(),
                total_count: total_count,
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
pub struct ReadHistoryDetailParam {
    pub history_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReadHistoryDetailResponse {
    pub success: bool,
    pub current_history: DocumentHistory,
    pub prev_history: Option<DocumentHistory>,
    pub message: String,
}

#[get("/doc/history")]
pub async fn read_document_history_detail(
    web::Query(query): web::Query<ReadHistoryDetailParam>,
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

    let result: Result<(DocumentHistory, Option<DocumentHistory>), diesel::result::Error> =
        (|| {
            let current_history = tb_document_history::table
                .inner_join(
                    tb_user::table.on(tb_user::dsl::id.eq(tb_document_history::dsl::writer_id)),
                )
                .filter(tb_document_history::dsl::id.eq(query.history_id))
                .select((
                    tb_document_history::dsl::id,
                    tb_document_history::dsl::content,
                    tb_document_history::dsl::char_count,
                    tb_document_history::dsl::increase,
                    tb_document_history::dsl::reg_utc,
                    tb_document_history::dsl::revision_number,
                    tb_document_history::dsl::writer_id,
                    tb_user::dsl::nickname,
                ))
                .get_result::<DocumentHistory>(connection)?;

            let prev_history = tb_document_history::table
                .inner_join(
                    tb_user::table.on(tb_user::dsl::id.eq(tb_document_history::dsl::writer_id)),
                )
                .filter(tb_document_history::dsl::id.ne(query.history_id))
                .filter(tb_document_history::dsl::reg_utc.le(current_history.reg_utc))
                .order(tb_document_history::dsl::reg_utc.desc())
                .limit(1)
                .select((
                    tb_document_history::dsl::id,
                    tb_document_history::dsl::content,
                    tb_document_history::dsl::char_count,
                    tb_document_history::dsl::increase,
                    tb_document_history::dsl::reg_utc,
                    tb_document_history::dsl::revision_number,
                    tb_document_history::dsl::writer_id,
                    tb_user::dsl::nickname,
                ))
                .get_result::<DocumentHistory>(connection)
                .ok();

            Ok((current_history, prev_history))
        })();

    match result {
        Ok((current_history, prev_history)) => {
            let response = ReadHistoryDetailResponse {
                success: true,
                current_history: current_history,
                prev_history: prev_history,
                message: "".into(),
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
pub struct RollbackDocParam {
    pub history_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RollbackDocResponse {
    pub success: bool,
    pub message: String,
}

#[post("/doc/history/rollback")]
pub async fn rollback_document_history(
    web::Json(body): web::Json<RollbackDocParam>,
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

    let result = connection.transaction(|| {
        let selected_history = tb_document_history::table
            .filter(tb_document_history::dsl::id.eq(body.history_id))
            .get_result::<SelectDocumentHistory>(connection)?;

        let (latest_history_char_count, latest_history_revision_number): (i64, i64) =
            tb_document_history::dsl::tb_document_history
                .filter(tb_document_history::dsl::document_id.eq(selected_history.document_id))
                .filter(tb_document_history::dsl::latest_yn.eq(true))
                .order(tb_document_history::dsl::reg_utc.desc())
                .limit(1)
                .select((
                    tb_document_history::dsl::char_count,
                    tb_document_history::dsl::revision_number,
                ))
                .get_result(connection)?;

        diesel::update(tb_document_history::dsl::tb_document_history)
            .filter(tb_document_history::dsl::document_id.eq(selected_history.document_id))
            .set(tb_document_history::dsl::latest_yn.eq(false))
            .execute(connection)?;

        let insert_history = InsertDocumentHistory {
            writer_id: auth.user_id,
            document_id: selected_history.document_id,
            content: selected_history.content,
            char_count: selected_history.char_count,
            increase: selected_history.increase - latest_history_char_count,
            rollback_id: Some(selected_history.id),
            revision_number: latest_history_revision_number + 1,
        };

        diesel::insert_into(tb_document_history::dsl::tb_document_history)
            .values(insert_history)
            .execute(connection)
    });

    match result {
        Ok(_) => {
            let response = RollbackDocResponse {
                success: true,
                message: "문서 되돌리기 성공".into(),
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
