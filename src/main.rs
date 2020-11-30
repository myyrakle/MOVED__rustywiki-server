use std::env;

use actix_web::web::Data;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use std::sync::Mutex;

#[macro_use]
extern crate diesel;
use diesel::*;
//use diesel::table;
//use diesel::prelude::*;

use serde::{Deserialize, Serialize};

mod middleware;
mod routes;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Test {
    pub id: i64,
    pub text: String,
}

mod schema;

#[path = "value/auth.rs"]
mod auth_value;
use auth_value::AuthValue;

#[get("/test")]
async fn test(request: HttpRequest, _connection: Data<Mutex<PgConnection>>) -> impl Responder {
    let extensions = request.extensions();
    let auth: &AuthValue = extensions.get::<AuthValue>().unwrap();
    let text = if auth.is_authorized() {
        "인증됨"
    } else {
        "인증 안됨"
    };

    web::Json(text)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _args: Vec<String> = env::args().collect();

    let host = "localhost"; //&args[1];
    let port = 11111; //&args[2];
    let address = format!("{}:{}", host, port);

    let _ = listenfd::ListenFd::from_env();

    let db = Data::new(Mutex::new(establish_connection()));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(middleware::Logger::new())
            .wrap(middleware::Auth::new())
            .service(test)
            .service(routes::doc::create_doc)
            .service(routes::doc::update_doc)
            .service(routes::doc::read_doc)
            .service(routes::doc::delete_doc)
            .service(actix_files::Files::new("/", "static").show_files_listing())
    })
    .bind(address)?
    .run()
    .await
}
