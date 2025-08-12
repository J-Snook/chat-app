use chrono::Duration;
use bcrypt::DEFAULT_COST;

pub const REFRESH_TOKEN_DAYS: i64 = 30;
pub const REFRESH_TOKEN_DURATION: Duration = Duration::days(REFRESH_TOKEN_DAYS);
pub const ACCESS_TOKEN_SECONDS: i64 = 900;
pub const ACCESS_TOKEN_DURATION: Duration = Duration::seconds(ACCESS_TOKEN_SECONDS);

pub const HASHING_COST: u32 = DEFAULT_COST;
pub const DUMMY_PASSWORD_HASH: &str = "$2a$12$iZZzMqg1AiL6YLsZjvTp9ukg8jHmGGxmxjAL88jAGanp9eAgCzkWS";