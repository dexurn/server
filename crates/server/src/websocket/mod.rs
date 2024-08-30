use axum::{
    extract::{ws, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};

pub async fn websocket(
    ws: WebSocketUpgrade,
    Extension(pool): Extension<db::Pool>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| socket_handler(socket, pool))
}

async fn socket_handler(_stream: ws::WebSocket, _pool: db::Pool) {}
