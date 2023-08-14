use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub lvl: Option<i32>,
}

pub fn generate_token(user_id: i32, level: i32) -> String {
    let secret_key = "SECRET_KEY";
    let encoding_key = EncodingKey::from_secret(secret_key.as_bytes());

    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::weeks(2)).timestamp() as usize,
        lvl: Some(level),
    };

    encode(&Header::default(), &claims, &encoding_key).unwrap()
}
