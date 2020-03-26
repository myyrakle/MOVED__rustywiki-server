use actix_web::{get, post, put, delete};

#[post("/api/create_doc")]
async fn create_doc(_req: HttpRequest) -> impl Responder 
{
    "미구현"
}

#[get("/api/read_doc")]
async fn read_doc(_req: HttpRequest) -> impl Responder 
{
    "미구현"
}

#[put("/api/update_doc")]
async fn update_doc(_req: HttpRequest) -> impl Responder 
{
    "미구현"
}

#[delete("/api/delete_doc")]
async fn delete_doc(_req: HttpRequest) -> impl Responder 
{
    "미구현"
}