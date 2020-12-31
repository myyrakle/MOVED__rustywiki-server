use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerErrorResponse {
    pub success: bool, 
    pub message: String,
}

impl ServerErrorResponse {
    pub fn new() -> ServerErrorResponse {
        ServerErrorResponse {
            success: false, 
            message: "server error".into(),
        }
    }
}