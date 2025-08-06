use sqlx::{Pool, Postgres};
use crate::api::utils::api_errors::ApiError;
use crate::api::models::auth_models::{AuthPublicUser, AuthUser};
use crate::api::utils::consts::{DUMMY_PASSWORD_HASH, HASHING_COST};

pub struct AuthService;

impl AuthService {
    pub async fn register_user(pool: &Pool<Postgres>, username: &str, password: &str) -> Result<AuthPublicUser, ApiError> {
        let password_hash = AuthService::hash_password(password)?;
        let user = AuthService::insert_user(pool, username, &password_hash).await?;
        Ok(user)
    }

    pub async fn login_user(pool: &Pool<Postgres>, username: &str, password: &str) -> Result<(), ApiError> {
        let user = match Self::get_user_by_username(pool, username).await {
            Ok(user) => user,
            Err(ApiError::Database(sqlx::Error::RowNotFound)) => {
                let _ = bcrypt::verify("dummy password guess", DUMMY_PASSWORD_HASH); //Prevent timing attacks
                return Err(ApiError::Unauthorized("Invalid username or password".to_string()))
            },
            Err(err) => return Err(ApiError::from(err)),
        };
        if !bcrypt::verify(password, &user.password_hash)? {
            return Err(ApiError::Unauthorized("Invalid username or password".to_string()))
        }

        Ok(())
    }

    fn hash_password(password: &str) -> Result<String, ApiError> {
        Ok(bcrypt::hash(password, HASHING_COST)?)
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

    async fn get_user_by_username(pool: &Pool<Postgres>, username: &str) -> Result<AuthUser, ApiError> {
        let result = sqlx::query_as!(AuthUser,"SELECT id, username, password_hash FROM users WHERE username = $1",username)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

}