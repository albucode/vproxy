use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::response::status::NotFound;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: i64,
}

pub fn generate() -> Result<String, NotFound<String>> {
    let expiration = (Utc::now() + Duration::hours(1)).timestamp();

    let claims = Claims {
        sub: generate_session_id(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("development_secret".as_ref()),
    )
    .map_err(|_| NotFound(String::from("Failed generating JWT")))
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, NotFound<String>> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("development_secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| NotFound(String::from("Failed generating JWT")))
}

fn generate_session_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
