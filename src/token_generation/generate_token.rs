use std::env;

use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{encode, errors::Result, EncodingKey, Header};

use crate::models::Claims;

pub fn generate_token(user_id: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // Token valid for 24 hours
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    dotenv().ok();
    let secret = env::var("SECRET_KEY").expect("DATABASE_URL MUST BE SET");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
