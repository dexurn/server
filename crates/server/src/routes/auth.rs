use std::str::FromStr;

use axum::{Extension, Json};
use dex_core::public_key::PublicKey;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tokio_postgres::GenericClient;
use validator::Validate;

use crate::{
    config::Config,
    error::AppError,
    extractors::validate_json::ValidateJson,
    utilities::{jwt, validations::validate_pubkey},
};

#[derive(Debug, Deserialize, Validate)]
pub struct RequestHandshake {
    #[validate(custom(function = "validate_pubkey"))]
    pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseHandshake {
    message: String,
}

pub async fn handshake(
    Extension(pool): Extension<db::Pool>,
    ValidateJson(input): ValidateJson<RequestHandshake>,
) -> Result<Json<ResponseHandshake>, AppError> {
    let mut buff = [0u8; 24];
    thread_rng().fill(&mut buff);
    let nonce = bs58::encode(buff).into_string();
    let message = format!("Hi, welcome. Here is your nonce: {}", nonce);

    let client_obj = pool.get().await?;
    let client = client_obj.client();

    let mut date = time::OffsetDateTime::now_utc();
    date -= time::Duration::hours(1);

    let res = db::queries::sessions::get_after_date()
        .bind(client, &input.pubkey, &date)
        .all()
        .await?;

    if res.len() >= 10 {
        return Err(AppError::RateLimit(
            "Too many requests. Try again later.".to_string(),
        ));
    }

    let _ = db::queries::sessions::insert()
        .bind(client, &input.pubkey, &message)
        .await?;

    Ok(Json(ResponseHandshake { message }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct RequestLogin {
    #[validate(custom(function = "validate_pubkey"))]
    pubkey: String,
    signature: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseLogin {
    access_token: String,
}

pub async fn login(
    Extension(pool): Extension<db::Pool>,
    Extension(config): Extension<Config>,
    ValidateJson(input): ValidateJson<RequestLogin>,
) -> Result<Json<ResponseLogin>, AppError> {
    let client_obj = pool.get().await?;
    let client = client_obj.client();

    let session = db::queries::sessions::get_latest_by_user()
        .bind(client, &input.pubkey)
        .one()
        .await?;

    if (time::OffsetDateTime::now_utc() - session.created_at).as_seconds_f32() >= 600.0 {
        return Err(AppError::Timeout(
            "Session has expired, please log in again.".to_string(),
        ));
    }

    if session.is_used {
        return Err(AppError::InvalidSession(
            "Session has already been used.".to_string(),
        ));
    }

    let message = session.message.as_bytes();
    let signature = bs58::decode(input.signature)
        .into_vec()
        .map_err(|_| AppError::Validation("Invalid Signature".to_string()))?;
    let pubkey = PublicKey::from_str(&input.pubkey)
        .map_err(|_| AppError::Validation("Invalid Publickey".to_string()))?;

    pubkey
        .verify(message, &signature)
        .map_err(|_| AppError::Unauthorized)?;

    let access_token = jwt::create_token(&config.jwt_secret, input.pubkey)?;

    let _ = db::queries::sessions::set_as_used()
        .bind(client, &session.id)
        .await?;

    Ok(Json(ResponseLogin { access_token }))
}
