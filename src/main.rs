use std::env;

use actix_web::{App, HttpServer};

mod middleware;
mod routes;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _args: Vec<String> = env::args().collect();

    let host = "127.0.0.1"; //&args[1];
    let port = 11111; //&args[2];

    let _ = listenfd::ListenFd::from_env();

    

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new())
            .service(actix_files::Files::new("/", "static").show_files_listing())
            .service(routes::doc::test)
            .service(routes::doc::create_doc)
            .service(routes::doc::update_doc)
            .service(routes::doc::read_doc)
            .service(routes::doc::delete_doc)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
