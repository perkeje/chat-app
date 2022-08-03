use crate::models::auth::user::User;
use chrono::prelude::*;
use jsonwebtoken;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn generate(user: &User) -> String {
    let secret = match dotenv::var("JWT_SECRET") {
        Ok(data) => data,
        Err(_) => String::from(""),
    };

    let duration: i64 = match dotenv::var("JWT_LIFETIME") {
        Ok(data) => data,
        Err(_) => String::from("300"),
    }
    .parse()
    .unwrap();

    let expire = Utc::now() + chrono::Duration::seconds(duration);

    let claims = Claims {
        sub: String::from(&user.id),
        email: String::from(&user.email),
        exp: expire.timestamp(),
        iat: Utc::now().timestamp(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap_or_default()
}

pub fn verify(token: String) -> Result<User, jsonwebtoken::errors::Error> {
    let secret = match dotenv::var("JWT_SECRET") {
        Ok(data) => data,
        Err(_) => String::from(""),
    };

    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;
    Ok(User::from_jwt(&token_data.claims))
}
