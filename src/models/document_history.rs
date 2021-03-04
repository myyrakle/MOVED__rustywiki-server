use crate::lib;
use crate::schema::tb_user;

#[derive(Insertable, Debug)]
#[table_name = "tb_document_history"]
pub struct InsertDocumentHistory {
    pub writer_id: i64,
    pub document_id: i64,
    pub filepath: i64,
    pub increase: i64,
}

#[derive(Queryable)]
#[table_name = "tb_document_history"]
pub struct SelectDocumentHistory {
    pub id: i64,
    pub writer_id: i64,
    pub document_id: i64,
    pub filepath: String,
    pub increase: i64,
    pub reg_utc: i64,
}
