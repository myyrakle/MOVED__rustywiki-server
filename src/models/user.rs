use super::super::schema::{tb_user};

#[derive(Insertable)]
#[table_name="tb_user"]
pub struct InsertUser {
    pub email: String, 
    pub password: String, 
    pub nickname: String, 
}

impl InsertUser { 
    pub fn new(email: String, password: String, nickname: String) -> InsertUser {
        // 패스워드 해싱 필요
        InsertUser{
            email: email,
            password: password,
            nickname: nickname,
        }
    }
}