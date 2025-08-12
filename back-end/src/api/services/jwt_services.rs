use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::api::models::jwt_models::TokenClaims;
use crate::api::utils::api_errors::ApiError;

pub struct JwtService;

impl JwtService {
    pub fn encode_token(token_claims: &TokenClaims, jwt_secret: &str) -> Result<String,ApiError> {
        Ok(jsonwebtoken::encode(&Header::default(),token_claims,&EncodingKey::from_secret(jwt_secret.as_ref()))?)
    }

    pub fn verify_token(token: &str, jwt_secret: &str) -> Result<TokenClaims,ApiError> {
        match jsonwebtoken::decode::<TokenClaims>(token,&DecodingKey::from_secret(jwt_secret.as_ref()),&Validation::new(Algorithm::HS256)) {
            Ok(decoded_token) => Ok(decoded_token.claims),
            Err(err) => Err(ApiError::from(err))
        }
    }
}