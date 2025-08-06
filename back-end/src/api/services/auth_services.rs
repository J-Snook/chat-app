use sqlx::{Pool, Postgres};
use crate::api::utils::api_errors::ApiError;
use crate::api::models::auth_models::AuthPublicUser;

pub struct AuthService;

impl AuthService {
    pub async fn register_user(pool: &Pool<Postgres>, username: &str, password: &str) -> Result<AuthPublicUser, ApiError> {
        let password_hash = AuthService::hash_password(password)?;
        let user = AuthService::insert_user(pool, username, &password_hash).await?;
        Ok(user)
    }

    fn hash_password(password: &str) -> Result<String, ApiError> {
        Ok(bcrypt::hash(password, bcrypt::DEFAULT_COST)?)
    }

    async fn insert_user(pool: &Pool<Postgres>,username: &str, password_hash: &str) -> Result<AuthPublicUser, ApiError> {
        let result = sqlx::query_as!(AuthPublicUser,"INSERT INTO users(username, password_hash) VALUES ($1, $2) RETURNING id, username",username,password_hash)
            .fetch_one(pool)
            .await;
        match result {
            Ok(user) => Ok(user),
            Err(err) => {
                if let Some(db_err) = err.as_database_error() {
                    if let Some(code) = db_err.code() {
                        if code == "23505" {
                            return Err(ApiError::Conflict(format!("User with username {} already exists", username)));
                        }
                    }
                }
                Err(ApiError::from(err))
            }
        }
    }
}