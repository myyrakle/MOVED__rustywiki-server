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
    pub fn new(email: String, password: String, nickname: String) -> InsertUser {
        use uuid::Uuid;
        let salt = Uuid::new_v4().to_string();

        let password = lib::hash(password + &salt);

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