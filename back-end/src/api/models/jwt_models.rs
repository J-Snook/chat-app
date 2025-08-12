use crate::api::services::cookies_service::CookieType;
use crate::api::utils::api_errors::ApiError;
use crate::api::utils::consts::{ACCESS_TOKEN_DURATION, REFRESH_TOKEN_DURATION};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i64,
    pub exp: i64,
    pub iat: i64,
    pub jti: Uuid,
    pub cookie_type: CookieType
}

impl TokenClaims {
    pub fn new_refresh_token(cookie_type:CookieType,user_id: &i64) -> Self {
        let current_time = Utc::now();
        let iat = current_time.timestamp();
        let exp = (current_time + REFRESH_TOKEN_DURATION).timestamp();
        Self {
            sub: user_id.clone(),
            exp,
            iat,
            jti: Uuid::new_v4(),
            cookie_type
        }
    }

    pub fn new_access_token(cookie_type:CookieType,user_id: i64, refresh_uuid: Uuid) -> Self {
        let current_time = Utc::now();
        let iat = current_time.timestamp();
        let exp = (current_time + ACCESS_TOKEN_DURATION).timestamp();
        Self {
            sub: user_id,
            exp,
            iat,
            jti: refresh_uuid,
            cookie_type
        }
    }

    pub fn get_exp_datetime(&self) -> Result<DateTime<Utc>, ApiError> {
        DateTime::from_timestamp(self.exp, 0).ok_or(ApiError::ValidationError("Failed to get timestamp from timestamp".to_string()))
    }
}
