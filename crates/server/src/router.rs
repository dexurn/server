use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};

use crate::{
    config::Config,
    middleware::protected_routes::protected_routes,
    routes::{
        auth::{handshake, login},
        ping::ping,
    },
    websocket::websocket,
};

pub fn create_router(config: Config, pool: db::Pool) -> Router {
    Router::new()
        .route("/websocket", get(websocket))
        .route_layer(middleware::from_fn(protected_routes))
        .route("/ping", get(ping))
        .route("/auth/login", post(login))
        .route("/auth/handshake", post(handshake))
        .layer(Extension(config))
        .layer(Extension(pool))
}
