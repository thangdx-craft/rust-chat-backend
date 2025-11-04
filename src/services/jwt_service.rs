use crate::errors::{AppError, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub exp: usize, // expiration time
}

#[derive(Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
    
    fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret.as_bytes())
    }
    
    fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_bytes())
    }

    pub fn generate_token(&self, user_id: i32, email: &str) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: expiration,
        };

        encode(&Header::default(), &claims, &self.encoding_key())
            .map_err(|_| AppError::InternalServerError)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key(), &Validation::default())
            .map(|data| data.claims)
            .map_err(|err| {
                tracing::warn!("Token verification failed: {:?}", err);
                AppError::InvalidToken
            })
    }
}
