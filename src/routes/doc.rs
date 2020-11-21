use actix_web::{
    get,
    post,
    put,
    delete,
    Responder,
    HttpRequest,
};

#[get("/test")]
pub async fn test(_req: HttpRequest) -> impl Responder
{
    "test"
}

#[post("/document")]
pub async fn create_doc(_req: HttpRequest) -> impl Responder
{
    "unimplemented"
}

#[get("/document")]
pub async fn read_doc(_req: HttpRequest) -> impl Responder
{
    "unimplemented"
}

#[put("/api/document")]
pub async fn update_doc(_req: HttpRequest) -> impl Responder
{
    "unimplemented"
}

#[delete("/api/document")]
pub async fn delete_doc(_req: HttpRequest) -> impl Responder
{
    "unimplemented"
}