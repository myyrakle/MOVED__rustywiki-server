use crate::schema::{tb_refresh_token};
use crate::lib;

#[derive(Insertable, Debug)]
#[table_name="tb_refresh_token"]
pub struct InsertRefreshToken {
    pub token_value: String, 
    pub user_id: i64,
}

#[derive(Queryable)]
pub struct SelectRefreshToken {
    pub token_value: String, 
    pub user_id: i64,
    pub dead_yn: bool,
}