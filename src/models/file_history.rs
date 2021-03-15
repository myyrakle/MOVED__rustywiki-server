use crate::schema::tb_file_history;

#[derive(Insertable, Debug)]
#[table_name = "tb_file_history"]
pub struct InsertFileHistory {
    pub writer_id: i64,
    pub file_id: i64,
    pub filepath: String
}

#[derive(Queryable)]
pub struct SelectFileHistory {
    pub id: i64,
    pub writer_id: i64,
    pub file_id: i64,
    pub filepath: String,
    pub use_yn: bool,
    pub reg_time: i64,
}
