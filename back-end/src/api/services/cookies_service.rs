use crate::api::models::jwt_models::TokenClaims;
use crate::api::services::jwt_services::JwtService;
use crate::api::utils::api_errors::ApiError;
use crate::api::utils::consts::{ACCESS_TOKEN_DURATION, REFRESH_TOKEN_DURATION};
use actix_web::cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

pub struct CookieService;

#[derive(Serialize, Deserialize)]
pub enum CookieType {
    Refresh,
    Access,
}

impl CookieService {
    pub async fn create_refresh_token(jwt_secret:&str,pool: &Pool<Postgres>, user_id:i64) -> Result<Cookie<'static>,ApiError> {
        let claims = TokenClaims::new(CookieType::Refresh,user_id);
        Self::insert_refresh_uuid_to_db(pool,&claims).await?;
        let encoded_token = JwtService::encode_token(&claims,jwt_secret)?;
        Ok(CookieService::create_cookie(CookieType::Refresh, encoded_token)?)
    }

    pub fn create_cookie(cookie_type: CookieType, value: String) -> Result<Cookie<'static>, ApiError> {
        let (name, path, max_age) = match cookie_type {
            CookieType::Refresh => ("refresh_token", "/auth/refresh", REFRESH_TOKEN_DURATION),
            CookieType::Access => ("access_token", "/", ACCESS_TOKEN_DURATION),
        };
        let cookie = Cookie::build(name, value)
            .path(path)
            .max_age(time::Duration::seconds(max_age.num_seconds()))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .finish();
        Ok(cookie)
    }

    async fn insert_refresh_uuid_to_db(pool: &Pool<Postgres>,token_claims: &TokenClaims) -> Result<(), ApiError> {
        sqlx::query!("INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1,$2,$3);",token_claims.sub,token_claims.jti,token_claims.get_exp_datetime()?).execute(pool).await?;
        Ok(())
    }
}