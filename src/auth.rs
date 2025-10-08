use chrono::TimeDelta;

pub mod client;
pub mod guard;

pub const SESSION_TOKEN_COOKIE_NAME: &str = "session_token";
pub const SESSION_TTL: TimeDelta = TimeDelta::hours(3);
