use actix_web::{
    get,
    post,
    put,
    delete,
    Responder,
    HttpRequest,
};

#[post("/api/create_doc")]
pub async fn create_doc(_req: HttpRequest) -> impl Responder
{
    "unimplemented"
}

#[get("/api/read_doc")]
pub async fn read_doc(_req: HttpRequest) -> impl Responder
{
    "unimplemented"
}

#[put("/api/update_doc")]
pub async fn update_doc(_req: HttpRequest) -> impl Responder
{
    "unimplemented"
}

#[delete("/api/delete_doc")]
pub async fn delete_doc(_req: HttpRequest) -> impl Responder
{
    "unimplemented"
}