// standard
use std::borrow::Borrow;
use std::sync::Mutex;

// thirdparty
use actix_web::{get, http::StatusCode, web::Data, HttpResponse, Responder};
use actix_web_validator::{Query, Validate};
use diesel::*;
use serde::{Deserialize, Serialize};

// in crate
use crate::response::ServerErrorResponse;
use crate::schema::tb_document;

#[derive(Deserialize, Validate, Debug)]
pub struct SearchDocParam {
    pub search_text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchDocResponse {
    pub success: bool,
    pub search_list: Vec<String>,
    pub message: String,
}

#[get("/doc/search")]
pub async fn search_doc(
    Query(query): Query<SearchDocParam>,
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

    let search_list: Result<Vec<String>, _> = tb_document::dsl::tb_document
        .filter(tb_document::dsl::title.like(query.search_text + "%"))
        .order(tb_document::dsl::title.asc())
        .select(tb_document::dsl::title)
        .get_results(connection);

    match search_list {
        Ok(search_list) => {
            let response = SearchDocResponse {
                success: true,
                search_list: search_list,
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
