use actix_web::{get, post};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
extern crate env_logger;

mod action;

fn init_logger()
{
    //logger setting
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> 
{
    init_logger();

    let mut listenfd = listenfd::ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(r#"요청:"%r" 상태:%s 처리시간:%Dms"#))
            .service(action::create_doc)
            .service(action::read_doc) 
            .service(actix_files::Files::new("/", "/static").show_files_listing())
    });

    //열려있으면 열려있던걸 재사용
    server = if let Some(li) = listenfd.take_tcp_listener(0).expect("실패") {
        server.listen(li)?    
    } else { //없으면 새로 엶
        server.bind("127.0.0.1:8000")?
    };

    server.run().await
}