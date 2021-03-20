use crate::schema::tb_document;

#[derive(Insertable, Debug)]
#[table_name = "tb_document"]
pub struct InsertDocument {
    pub title: String,
}

#[derive(Queryable)]
pub struct SelectDocument {
    pub id: i64,
    pub title: String,
    pub update_utc: i64,
    pub reg_utc: i64,
}
