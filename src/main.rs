#[macro_use]
extern crate diesel;

mod lib;
mod middleware;
mod routes;
mod schema;

#[path = "value/auth.rs"]
mod auth_value;
use auth_value::AuthValue;

use actix_web::web::Data;
use actix_web::{
    dev::Body, get, http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use diesel::*;
use std::sync::Mutex;

#[get("/test")]
async fn test(
    request: HttpRequest, /*, _connection: Data<Mutex<PgConnection>>*/
) -> impl Responder {
    //let auth_value = AuthValue::new();
    //request.extensions_mut().insert(auth_value);

    let extensions = request.extensions();
    let auth: &AuthValue = extensions.get::<AuthValue>().unwrap();
    let text = if auth.is_authorized() {
        "인증됨"
    } else {
        "인증 안됨"
    };

    //let a = AuthValue::new();
    //let a = [1, 3, 3, 4];

    text.to_string()
    //HttpResponse::build(StatusCode::OK).json(a)
}

#[get("/foo")]
async fn foo(_request: HttpRequest) -> impl Responder {
    "foo".to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _args: Vec<String> = std::env::args().collect();

    let host = "localhost"; //&args[1];
    let port = 11111; //&args[2];
    let address = format!("{}:{}", host, port);

    let _ = listenfd::ListenFd::from_env();

    let db = Data::new(Mutex::new(lib::establish_connection()));

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(middleware::Logger::new())
            .wrap(middleware::Auth::new())
            .service(routes::auth::get_token)
            .service(test)
            .service(foo)
            .service(routes::doc::create_doc)
            .service(routes::doc::update_doc)
            .service(routes::doc::read_doc)
            .service(routes::doc::delete_doc)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
    })
    .bind(address)?
    .run()
    .await
}
