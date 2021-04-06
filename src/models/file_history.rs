use crate::schema::tb_file_history;

#[derive(Insertable, Debug)]
#[table_name = "tb_file_history"]
pub struct InsertFileHistory {
    pub writer_id: i64,
    pub file_id: i64,
    pub content: Option<String>,
    pub char_count: i64,
    pub increase: i64,
}

#[derive(Queryable)]
pub struct SelectFileHistory {
    pub id: i64,
    pub writer_id: i64,
    pub file_id: i64,
    pub content: String,
    pub char_count: i64,
    pub increase: i64,
    pub reg_time: i64,
    pub latest_yn: bool,
}
