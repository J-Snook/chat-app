use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use crate::api::models::auth_models::AuthSentUserInfo;
use crate::api::services::auth_services::AuthService;
use crate::api::services::cookies_service::{CookieService, CookieType};
use crate::api::utils::api_errors::ApiError;
use crate::AppData;

pub async fn handle_register(app_data: Data<AppData>, user_info: Json<AuthSentUserInfo>) -> Result<impl Responder, ApiError> {
    let result = AuthService::register_user(&app_data.pool,&user_info.username,&user_info.password).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn handle_login(app_data: Data<AppData>, user_info: Json<AuthSentUserInfo>) -> Result<impl Responder, ApiError> {
    let user = AuthService::login_user(&app_data.pool,&user_info.username,&user_info.password).await?;
    let (new_refresh_token,refresh_claims) = CookieService::create_refresh_token(&app_data.jwt_secret,&app_data.pool,&user.id).await?;
    let (new_access_token,_) = CookieService::create_access_token(&app_data.jwt_secret,&app_data.pool,&refresh_claims).await?;
    let is_logged_in_cookie = CookieService::create_cookie(CookieType::LoggedInCookie,"true".to_string())?;
    let response = HttpResponse::Ok()
        .cookie(new_refresh_token)
        .cookie(new_access_token)
        .cookie(is_logged_in_cookie)
        .json(user);
    Ok(response)
}

pub async fn handle_logout(app_data: Data<AppData>, req: HttpRequest) -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::NotImplemented().json("logged out"))
}

pub async fn handle_refresh(app_data: Data<AppData>, req: HttpRequest) -> Result<impl Responder, ApiError> {
    let refresh_token = match req.cookie("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Err(ApiError::Unauthorized("No refresh token found, please login again".to_string()))
    };
    let user_id = AuthService::refresh_token(&app_data.pool,&app_data.jwt_secret,&refresh_token).await?;
    let (new_refresh_token,refresh_claims) = CookieService::create_refresh_token(&app_data.jwt_secret,&app_data.pool,&user_id).await?;
    let (new_access_token,_) = CookieService::create_access_token(&app_data.jwt_secret,&app_data.pool,&refresh_claims).await?;
    let is_logged_in_cookie = CookieService::create_cookie(CookieType::LoggedInCookie,"true".to_string())?;
    let response = HttpResponse::Ok()
        .cookie(new_refresh_token)
        .cookie(new_access_token)
        .cookie(is_logged_in_cookie)
        .json("Refreshed Token");
    Ok(response)
}

pub async fn handle_me(app_data: Data<AppData>) -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::NotImplemented().json("logged out"))
}