use axum::{routing::get, Router};

use crate::routes::ping::ping;

pub fn create_router() -> Router {
    Router::new().route("/ping", get(ping))
}
