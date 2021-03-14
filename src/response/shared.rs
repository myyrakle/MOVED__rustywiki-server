use serde::{Deserialize, Serialize};

// status 500
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

// status 400
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BadParameter {
    pub success: bool, 
    pub message: String,
}

impl BadParameter {
    pub fn new() -> BadParameter {
        BadParameter {
            success: false, 
            message: "bad parameter".into(),
        }
    }
}

// status 401
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UnauthorizedResponse {
    pub success: bool, 
    pub message: String,
}

impl UnauthorizedResponse {
    pub fn new() -> UnauthorizedResponse {
        UnauthorizedResponse {
            success: false, 
            message: "you are unauthorized".into(),
        }
    }
}