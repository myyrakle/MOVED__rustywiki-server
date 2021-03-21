use crate::schema::tb_document_history;

#[derive(Insertable, Debug)]
#[table_name="tb_document_history"]
pub struct InsertDocumentHistory {
    pub writer_id: i64,
    pub document_id: i64,
    pub content: String,
    pub char_count: i64,
    pub increase: i64,
}

#[derive(Queryable)]
pub struct SelectDocumentHistory {
    pub id: i64,
    pub writer_id: i64,
    pub document_id: i64,
    pub content: String,
    pub char_count: i64,
    pub increase: i64,
    pub reg_utc: i64,
}
