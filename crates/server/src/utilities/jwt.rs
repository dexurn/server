use crate::error::AppError;
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    pubkey: String,
}

pub fn create_token(secret: &str, pubkey: String) -> Result<String, AppError> {
    let now = time::OffsetDateTime::now_utc();
    let expires_at = time::Duration::hours(1);
    let expires_at = now + expires_at;
    let exp = expires_at.unix_timestamp() as usize;
    let claims = Claims { exp, pubkey };
    let token_header = jwt::Header::default();
    let key = jwt::EncodingKey::from_secret(secret.as_bytes());

    jwt::encode(&token_header, &claims, &key).map_err(|_| {
        AppError::InternalError("There was an error, please try again later".to_string())
    })
}

pub fn validate_token(secret: &str, token: &str) -> Result<String, AppError> {
    let key = jwt::DecodingKey::from_secret(secret.as_bytes());
    let validation = jwt::Validation::new(jsonwebtoken::Algorithm::HS256);
    jwt::decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::Unauthorized,
            _ => {
                // eprintln!("Error validating token: {:?}", error);
                AppError::InternalError("Error validating token".to_string())
            }
        })
        .map(|data| data.claims.pubkey)
}
