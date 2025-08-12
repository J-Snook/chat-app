use jsonwebtoken::{EncodingKey, Header};
use crate::api::models::jwt_models::TokenClaims;
use crate::api::utils::api_errors::ApiError;

pub struct JwtService;

impl JwtService {
    pub fn encode_token(token_claims: &TokenClaims, jwt_secret: &str) -> Result<String,ApiError> {
        Ok(jsonwebtoken::encode(&Header::default(),token_claims,&EncodingKey::from_secret(jwt_secret.as_ref()))?)
    }
}