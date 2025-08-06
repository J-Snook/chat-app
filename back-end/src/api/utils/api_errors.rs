use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use bcrypt::BcryptError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorOutput {
    pub(crate) error: String,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized(String),
    #[error("Forbidden")]
    Forbidden(String),
    #[error("Not Found")]
    NotFound,
    #[error("Bad Request")]
    BadRequest(String),
    #[error("Conflict")]
    Conflict(String),

    #[error("Hashing Error")]
    Hashing(#[from] BcryptError),

    #[error("Database Error")]
    Database(#[from] sqlx::Error)
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ApiError::Unauthorized(message) => HttpResponse::Unauthorized().json(ErrorOutput {error: message.to_string() }),
            ApiError::Forbidden(message) => HttpResponse::Forbidden().json(ErrorOutput {error: message.to_string() }),
            ApiError::BadRequest(message) => HttpResponse::BadRequest().json(ErrorOutput {error: message.to_string() }),
            ApiError::Conflict(message) => HttpResponse::Conflict().json(ErrorOutput {error: message.to_string() }),
            ApiError::NotFound => HttpResponse::NotFound().finish(),
            _ => {
                eprintln!("Error: {:?}", self);
                HttpResponse::InternalServerError().body("An internal server error occurred")
            }
        }
    }
}