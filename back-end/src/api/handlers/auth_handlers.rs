use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use crate::api::models::auth_models::AuthSentUserInfo;
use crate::api::models::cookie_models::CookieType;
use crate::api::models::jwt_models::TokenClaims;
use crate::api::services::auth_services::AuthService;
use crate::api::services::cookies_service::{CookieService};
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
    let claims = get_claims(&req)?;
    AuthService::logout_user(&app_data.pool, &claims.jti).await?;
    let empty_refresh_cookie = CookieService::create_empty_cookie(CookieType::Refresh)?;
    let empty_access_cookie = CookieService::create_empty_cookie(CookieType::Access)?;
    let empty_logged_in_cookie = CookieService::create_empty_cookie(CookieType::LoggedInCookie)?;
    let response = HttpResponse::Ok()
        .cookie(empty_refresh_cookie)
        .cookie(empty_access_cookie)
        .cookie(empty_logged_in_cookie)
        .json("Logged out");
    Ok(response)
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

pub async fn handle_me(app_data: Data<AppData>, req: HttpRequest) -> Result<impl Responder, ApiError> {
    let claims = get_claims(&req)?;
    let user = AuthService::get_user_by_id(&app_data.pool,&claims.sub).await?;
    Ok(HttpResponse::Ok().json(user))
}

fn get_claims(req: &HttpRequest) -> Result<TokenClaims, ApiError> {
    match req.extensions().get::<TokenClaims>().cloned() {
        Some(claims) => Ok(claims),
        None => Err(ApiError::Unauthorized("No token found, please login again".to_string()))
    }
}