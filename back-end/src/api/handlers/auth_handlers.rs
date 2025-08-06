use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json};
use crate::api::models::auth_models::AuthSentUserInfo;
use crate::api::services::auth_services::AuthService;
use crate::api::utils::api_errors::ApiError;
use crate::AppData;

pub async fn handle_register(app_data: Data<AppData>, user_info: Json<AuthSentUserInfo>) -> Result<impl Responder, ApiError> {
    let result = AuthService::register_user(&app_data.pool,&user_info.username,&user_info.password).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn handle_login(app_data: Data<AppData>, user_info: Json<AuthSentUserInfo>) -> Result<impl Responder, ApiError> {
    AuthService::login_user(&app_data.pool,&user_info.username,&user_info.password).await?;
    Ok(HttpResponse::Ok().json("logged in"))
}