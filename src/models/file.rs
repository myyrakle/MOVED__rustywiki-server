use crate::schema::tb_file;

#[derive(Insertable, Debug)]
#[table_name = "tb_file"]
pub struct InsertFile {
    pub uploader_id: i64,
    pub title: String,
    pub filepath: String,
}

#[derive(Queryable)]
pub struct SelectFile {
    pub id: i64,
    pub uploader_id: i64,
    pub title: String,
    pub filepath: String,
    pub use_yn: bool,
    pub reg_time: i64,
    pub recent_history_id: Option<i64>,
}
