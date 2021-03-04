use actix_web::{delete, get, post, put, web::Data, HttpRequest, Responder};

#[get("/fftest")]
pub async fn test(_foo: Data<String>) -> impl Responder {
    ""
    //foo.into_inner().into_string();
}

#[post("/doc/document")]
pub async fn create_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}

#[get("/doc/document")]
pub async fn read_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}

#[put("/doc/document")]
pub async fn update_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}

#[delete("/doc/document")]
pub async fn delete_doc(_req: HttpRequest) -> impl Responder {
    "unimplemented"
}
