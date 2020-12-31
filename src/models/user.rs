use super::super::schema::{tb_user};
use super::super::lib;

#[derive(Insertable, Debug)]
#[table_name="tb_user"]
pub struct InsertUser {
    pub email: String, 
    pub password: String, 
    pub salt: String,
    pub nickname: String, 
}

impl InsertUser { 
    pub fn new(email: String, password: String, salt: String, nickname: String) -> InsertUser {
        let password = lib::hash(password);

        InsertUser{
            email: email,
            password: password,
            salt: salt,
            nickname: nickname,
        }
    }
}

#[derive(Queryable)]
pub struct SelectUser {
    pub id: i64, 
    pub email: String, 
    pub password: String,
    pub salt: String,  
    pub nickname: String, 
    pub user_type: String, 
    pub use_yn: bool,
    pub reg_time: i64,
}