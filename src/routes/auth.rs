use actix_web::{delete, get, post, put, web::Data, HttpRequest, Responder};

#[path = "../lib/jwt.rs"]
mod jwt;

#[get("/login")]
pub async fn test(_foo: Data<String>) -> impl Responder {
    web::Json(jwt::sign(2, "foo".into()))
    //foo.into_inner().into_string();
}

#[post("/document")]
pub async fn create_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}

#[get("/document")]
pub async fn read_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}

#[put("/api/document")]
pub async fn update_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}

#[delete("/api/document")]
pub async fn delete_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}
