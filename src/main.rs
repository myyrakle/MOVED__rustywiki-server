use std::env;

use actix_web::web::Data;
use actix_web::{
    dev::{Extensions, ServiceRequest, ServiceResponse},
    get, web, App, HttpRequest, HttpServer, Responder,
};
use std::sync::Mutex;

#[macro_use]
extern crate diesel;
use diesel::*;
//use diesel::table;
//use diesel::prelude::*;

use serde::{Deserialize, Serialize};

mod middleware;
use middleware::AuthInfo;
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

#[get("/test")]
async fn test(request: HttpRequest, connection: Data<Mutex<PgConnection>>) -> impl Responder {
    use schema::test;
    use std::borrow::Borrow;

    //let connection = connection.lock().unwrap();
    //let connection:&PgConnection = Borrow::borrow(&connection);
    //let results = test::dsl::test.load::<Test>(connection).unwrap();

    let auth: &AuthInfo = request.extensions().get::<AuthInfo>().unwrap();
    //println!("?{}", auth.is_authorized());

    web::Json("results")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _args: Vec<String> = env::args().collect();

    let host = "localhost"; //&args[1];
    let port = 11111; //&args[2];
    let address = format!("{}:{}", host, port);

    let _ = listenfd::ListenFd::from_env();

    let db = Data::new(Mutex::new(establish_connection()));

    //let auth_info = Data::new(Mutex::new());

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
