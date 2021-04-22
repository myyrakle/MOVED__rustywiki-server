// standard
use std::sync::Mutex;

// thirdparty
#[macro_use]
extern crate diesel;
use actix_web::web::Data;
use actix_web::{get, App, HttpRequest, HttpServer, Responder};

// in crate
mod lib;
mod middleware;
mod models;
mod response;
mod routes;
mod schema;
mod value;

use lib::AuthValue;

#[get("/")]
async fn root(
    request: HttpRequest, /*, _connection: Data<Mutex<PgConnection>>*/
) -> impl Responder {
    let extensions = request.extensions();
    let auth: &AuthValue = extensions.get::<AuthValue>().unwrap();
    let text = if auth.is_authorized() {
        "인증됨"
    } else {
        "인증 안됨"
    };

    text.to_string()
}

use diesel::*;
use std::borrow::Borrow;

#[derive(Queryable, Debug)]
pub struct SelectTest {
    pub id: i64,
    pub text: Option<String>,
}

#[get("/test")]
async fn test(connection: Data<Mutex<PgConnection>>) -> impl Responder {
    let connection = match connection.lock() {
        Err(_) => {
            log::error!("database connection lock error");
            return "error".to_string();
        }
        Ok(connection) => connection,
    };
    let _connection: &PgConnection = Borrow::borrow(&connection);

    "".to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _args: Vec<String> = std::env::args().collect();

    //let host = "192.168.1.2"; //&args[1];
    let host = "0.0.0.0";
    let port = 11111; //&args[2];
    let address = format!("{}:{}", host, port);

    let _ = listenfd::ListenFd::from_env();

    let db = Data::new(Mutex::new(lib::establish_connection()));
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone()) //데이터베이스 커넥션 객체 등록
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin("http://localhost:11111")
                    .allowed_origin("http://127.0.0.1:11111")
                    .allowed_origin("http://125.133.80.144:11111")
                    .allowed_origin("http://222.121.122.142:5500")
                    .allowed_origin("http://127.0.0.1:5500")
                    .allowed_origin("http://127.0.0.1:5500/")
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://127.0.0.1:3000/")
                    .allowed_origin("http://localhost:3000/")
                    .allowed_origin("http://192.168.1.2:11111")
                    .supports_credentials(),
            )
            .wrap(middleware::Logger::new()) //로깅용 미들웨어
            .service(root)
            .service(test)
            .service(routes::auth::signup)
            .service(routes::auth::login)
            .service(routes::auth::logout)
            .service(routes::auth::refresh)
            .service(routes::file::upload_file)
            .service(routes::user::my_info)
            .service(routes::user::close_my_account)
            .service(routes::doc::write_doc)
            .service(routes::doc::read_doc)
            .service(routes::history::read_document_history_list)
            .service(routes::history::read_document_history_detail)
            .service(routes::history::rollback_document_history)
            .service(routes::search::search_doc)
            .service(routes::debate::create_debate)
            .service(routes::debate::write_comment)
            .service(routes::debate::get_debate_list)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
            .wrap(middleware::Auth::new())
    })
    .bind(address)?
    .run()
    .await
}
