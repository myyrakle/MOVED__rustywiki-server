use crate::schema::tb_debate;

#[derive(Insertable, Debug)]
#[table_name = "tb_debate"]
pub struct InsertDebate {
    pub document_id: i64,
    pub writer_id: i64,
    pub subject: String,
    pub content: String,
}

#[derive(Queryable)]
pub struct SelectDebate {
    pub id: i64,
    pub document_id: i64,
    pub writer_id: i64,
    pub subject: String,
    pub content: String,
    pub reg_utc: i64,
    pub use_yn: bool,
}
