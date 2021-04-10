// standard
use std::borrow::Borrow;
use std::sync::Mutex;

// thirdparty
use actix_web::{get, http::StatusCode, web, web::Data, HttpRequest, HttpResponse, Responder};
use diesel::*;
use serde::{Deserialize, Serialize};

// in crate
//use crate::lib::AuthValue;
//use crate::models::SelectUser;
use crate::models::{SelectDocument, SelectDocumentHistory};
use crate::response::ServerErrorResponse;
use crate::schema::{tb_document, tb_document_history};
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

#[get("/doc/history")]
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

    let result: Result<(Vec<SelectDocumentHistory>, i64), diesel::result::Error> = (|| {
        let document: SelectDocument = tb_document::dsl::tb_document
            .filter(tb_document::dsl::title.eq(&query.title))
            .get_result(connection)?;

        let history_list: Vec<_> = tb_document_history::dsl::tb_document_history
            .filter(tb_document_history::dsl::document_id.eq(document.id))
            .order(tb_document_history::dsl::reg_utc.desc())
            .offset(offset)
            .limit(limit)
            .get_results::<SelectDocumentHistory>(connection)?;

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
                list: history_list.into_iter().map(|e| e.into()).collect(),
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
    pub content: DocumentHistory,
    pub prev_content: Option<DocumentHistory>,
    pub message: String,
}

#[get("/doc/history/detail")]
pub async fn read_document_history_detail(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}
