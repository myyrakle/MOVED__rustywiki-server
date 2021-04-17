use crate::schema::tb_debate_comment;

#[derive(Insertable, Debug)]
#[table_name = "tb_debate_comment"]
pub struct InsertDebateComment {
    pub document_id: i64,
    pub writer_id: i64,
    pub subject: String,
    pub content: String,
}

#[derive(Queryable)]
pub struct SelectDebateComment {
    pub id: i64,
    pub document_id: i64,
    pub writer_id: i64,
    pub subject: String,
    pub content: String,
    pub reg_utc: i64,
}
