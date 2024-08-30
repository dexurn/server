use std::str::FromStr;

use axum::{extract::Request, middleware::Next, response::Response, Extension};
use dex_core::public_key::PublicKey;
use hyper::HeaderMap;

use crate::{config::Config, error::AppError, utilities::jwt};

#[derive(Debug, Clone)]
pub struct User {
    pub pubkey: PublicKey,
}

pub async fn protected_routes(
    Extension(config): Extension<Config>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let header_token = if let Some(token) = headers.get("Authorization") {
        token.to_str().map_err(|error| {
            eprintln!("Error extracting token from headers: {:?}", error);
            AppError::InternalError("Error reading token".to_string())
        })?
    } else {
        return Err(AppError::Unauthorized);
    };

    let pubkey = jwt::validate_token(&config.jwt_secret, header_token)?;
    request.extensions_mut().insert(User {
        pubkey: PublicKey::from_str(&pubkey)
            .map_err(|_| AppError::Validation("Invalid Publickey".to_string()))?,
    });
    Ok(next.run(request).await)
}
