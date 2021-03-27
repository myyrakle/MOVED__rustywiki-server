// standard
use std::borrow::Borrow;
use std::sync::Mutex;

// thirdparty
use actix_web::{delete, get, http::StatusCode, web::Data, HttpRequest, HttpResponse, Responder};
use diesel::*;
use serde::{Deserialize, Serialize};

// in crate
use crate::lib::AuthValue;
use crate::models::SelectUser;
use crate::response::{ServerErrorResponse, UnauthorizedResponse};
use crate::schema::tb_user;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyInfoResponse {
    pub success: bool,
    pub message: String,
    pub email: String,
    pub nickname: String,
    pub reg_time: i64,
}

// 내 정보 획득
#[get("/user/my-info")]
pub async fn my_info(
    request: HttpRequest,
    connection: Data<Mutex<PgConnection>>,
) -> impl Responder {
    let connection = match connection.lock() {
        Err(error) => {
            log::error!("database connection lock error: {:?}", error);
            let response = ServerErrorResponse::new();
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
        }
        Ok(connection) => connection,
    };
    let connection: &PgConnection = Borrow::borrow(&connection);

    // 미인증 접근 거부
    let extensions = request.extensions();
    let nonauth = AuthValue::new();
    let auth: &AuthValue = extensions.get::<AuthValue>().unwrap_or(&nonauth);
    if !auth.is_authorized() {
        let response = UnauthorizedResponse::new();
        return HttpResponse::build(StatusCode::UNAUTHORIZED).json(response);
    }

    let select_user = tb_user::dsl::tb_user
        .filter(tb_user::dsl::id.eq(auth.user_id))
        .filter(tb_user::dsl::use_yn.eq(true))
        .get_result::<SelectUser>(connection);

    match select_user {
        Ok(user) => {
            let response = MyInfoResponse {
                success: true,
                message: "success".into(),
                email: user.email.clone(),
                nickname: user.nickname.clone(),
                reg_time: user.reg_time,
            };
            HttpResponse::build(StatusCode::OK).json(response)
        }
        Err(error) => {
            log::error!("select user error: {:?}", error);
            let response = ServerErrorResponse::new();
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CloseMyAccountResponse {
    pub success: bool,
    pub message: String,
}

// 회원탈퇴
#[delete("/user/close-my-account")]
pub async fn close_my_account(
    request: HttpRequest,
    connection: Data<Mutex<PgConnection>>,
) -> impl Responder {
    let connection = match connection.lock() {
        Err(error) => {
            log::error!("database connection lock error: {:?}", error);
            let response = ServerErrorResponse::new();
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response);
        }
        Ok(connection) => connection,
    };
    let connection: &PgConnection = Borrow::borrow(&connection);

    // 미인증 접근 거부
    let extensions = request.extensions();
    let nonauth = AuthValue::new();
    let auth: &AuthValue = extensions.get::<AuthValue>().unwrap_or(&nonauth);
    if !auth.is_authorized() {
        let response = UnauthorizedResponse::new();
        return HttpResponse::build(StatusCode::UNAUTHORIZED).json(response);
    }

    let update_filter = tb_user::dsl::tb_user
        .filter(tb_user::dsl::id.eq(auth.user_id))
        .filter(tb_user::dsl::use_yn.eq(true));

    let update_query = diesel::update(update_filter).set(tb_user::dsl::use_yn.eq(false));

    let result = update_query.execute(connection);

    match result {
        Ok(_) => {
            let response = CloseMyAccountResponse {
                success: true,
                message: "success".into(),
            };
            HttpResponse::build(StatusCode::OK).json(response)
        }
        Err(error) => {
            log::error!("회원탈퇴 쿼리 오류: {:?}", error);
            let response = ServerErrorResponse::new();
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(response)
        }
    }
}
