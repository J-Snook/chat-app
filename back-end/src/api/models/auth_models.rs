use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthRegisterUserInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthPublicUser {
    pub id: i64,
    pub username: String,
}

#[derive(Serialize)]
pub struct AuthUser {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

impl From<AuthUser> for AuthPublicUser {
    fn from(value: AuthUser) -> Self {
        AuthPublicUser {
            id: value.id,
            username: value.username,
        }
    }
}