// standard
use std::sync::Mutex;

// thirdparty
#[macro_use]
extern crate diesel;
use actix_web::web::Data;
use actix_web::{
    get, App, HttpRequest, HttpServer, Responder,
};

// in crate
mod middleware;
mod routes;
mod schema;
mod models;
mod response;
mod lib;

use lib::{AuthValue};

#[get("/")]
async fn test(
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
            .app_data(db.clone())
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin("http://localhost:11111")
                    .allowed_origin("http://127.0.0.1:11111")
                    .allowed_origin("http://125.133.80.144:11111")
                    .supports_credentials()
                )
            .wrap(middleware::Logger::new())
            .service(routes::auth::signup)
            .service(routes::auth::login)
            .service(routes::image::image_upload)
            .service(test)
            .service(routes::doc::create_doc)
            .service(routes::doc::update_doc)
            .service(routes::doc::read_doc)
            .service(routes::doc::delete_doc)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
            .wrap(middleware::Auth::new())
    })
    .bind(address)?
    .run()
    .await
}
