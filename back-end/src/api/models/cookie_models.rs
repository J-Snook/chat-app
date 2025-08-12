use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum CookieType {
    Refresh,
    Access,
    LoggedInCookie
}