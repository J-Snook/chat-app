use crate::api::models::cookie_models::CookieType;
use crate::api::models::jwt_models::TokenClaims;
use crate::api::services::jwt_services::JwtService;
use crate::api::utils::api_errors::ApiError;
use crate::api::utils::consts::{ACCESS_TOKEN_DURATION, REFRESH_TOKEN_DURATION};
use actix_web::cookie::{Cookie, SameSite};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct CookieService;

impl CookieService {
    pub async fn create_refresh_token(jwt_secret:&str,pool: &Pool<Postgres>, user_id: &i64) -> Result<(Cookie<'static>, TokenClaims),ApiError> {
        let claims = TokenClaims::new_refresh_token(CookieType::Refresh,user_id);
        Self::insert_refresh_uuid_to_db(pool,&claims).await?;
        let encoded_token = JwtService::encode_token(&claims,jwt_secret)?;
        Ok((CookieService::create_cookie(CookieType::Refresh, encoded_token)?,claims))
    }

    pub async fn create_access_token(jwt_secret:&str,pool: &Pool<Postgres>, refresh_token: &TokenClaims) -> Result<(Cookie<'static>, TokenClaims),ApiError> {
        let claims = TokenClaims::new_access_token(CookieType::Access,refresh_token.sub,refresh_token.jti);
        let encoded_token = JwtService::encode_token(&claims,jwt_secret)?;
        Ok((CookieService::create_cookie(CookieType::Access, encoded_token)?,claims))
    }

    pub fn create_cookie(cookie_type: CookieType, value: String) -> Result<Cookie<'static>, ApiError> {
        let (name, path, max_age,http_only,secure) = match cookie_type {
            CookieType::Refresh => ("refresh_token", "/api/auth/refresh", REFRESH_TOKEN_DURATION,true,true),
            CookieType::Access => ("access_token", "/api/", ACCESS_TOKEN_DURATION,true,true),
            CookieType::LoggedInCookie => ("is_logged_in", "/api/", REFRESH_TOKEN_DURATION,false,false),
        };
        let cookie = Cookie::build(name, value)
            .path(path)
            .max_age(time::Duration::seconds(max_age.num_seconds()))
            .http_only(http_only)
            .secure(secure)
            .same_site(SameSite::Strict)
            .finish();
        Ok(cookie)
    }

    pub fn create_empty_cookie(cookie_type: CookieType) -> Result<Cookie<'static>, ApiError> {
        let (name, path,http_only,secure) = match cookie_type {
            CookieType::Refresh => ("refresh_token", "/api/auth/refresh",true,true),
            CookieType::Access => ("access_token", "/api/",true,true),
            CookieType::LoggedInCookie => ("is_logged_in", "/api/",false,false),
        };
        let cookie = Cookie::build(name, "")
            .path(path)
            .max_age(time::Duration::seconds(0))
            .http_only(http_only)
            .secure(secure)
            .same_site(SameSite::Strict)
            .finish();
        Ok(cookie)
    }

    async fn insert_refresh_uuid_to_db(pool: &Pool<Postgres>,token_claims: &TokenClaims) -> Result<(), ApiError> {
        sqlx::query!("INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1,$2,$3);",token_claims.sub,token_claims.jti,token_claims.get_exp_datetime()?).execute(pool).await?;
        Ok(())
    }

    pub async fn check_token_in_database(pool: &Pool<Postgres>,token_id: &Uuid) -> Result<(), ApiError> {
        match sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM refresh_tokens WHERE token = $1 AND expires_at > NOW())",
        token_id
        ).fetch_one(pool).await? {
            Some(true) => Ok(()),
            Some(false) => Err(ApiError::Unauthorized("Refresh token expired or invalid".into())),
            _ => Err(ApiError::Other(String::from("validate_refresh_token_in_db issue, None returned."))),
        }
    }

    pub async fn revoke_refresh_token(pool: &Pool<Postgres>,token_id: &Uuid) -> Result<(), ApiError> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE token=$1;",*token_id).execute(pool).await?;
        Ok(())
    }
}