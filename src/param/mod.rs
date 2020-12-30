use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupParam {
    pub email: String, 
    pub password: String, 
    pub nickname: String,
}