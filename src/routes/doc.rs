use actix_web::{
    web::{Data},
    get,
    post,
    put,
    delete,
    Responder,
    HttpRequest,
};

#[get("/fftest")]
pub async fn test(_foo: Data<String>) -> impl Responder
{
    ""
    //foo.into_inner().into_string();
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