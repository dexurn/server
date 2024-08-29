use axum::{
    routing::{get, post},
    Extension, Router,
};

use crate::{
    config::Config,
    routes::{
        auth::{handshake, login},
        ping::ping,
    },
};

pub fn create_router(config: Config, pool: db::Pool) -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/auth/login", post(login))
        .route("/auth/handshake", post(handshake))
        .layer(Extension(config))
        .layer(Extension(pool))
}
