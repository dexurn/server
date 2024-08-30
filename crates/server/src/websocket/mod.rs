pub mod event;

use axum::{
    extract::{ws, State, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};

use crate::{middleware::protected_routes::User, state::AppState};

pub async fn websocket(
    ws: WebSocketUpgrade,
    Extension(pool): Extension<db::Pool>,
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| socket_handler(socket, pool, user, state))
}

async fn socket_handler(_stream: ws::WebSocket, _pool: db::Pool, _user: User, _state: AppState) {}
