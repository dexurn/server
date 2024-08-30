use std::net::SocketAddr;

use config::Config;
use router::create_router;

pub mod config;
pub mod error;
pub mod extractors;
pub mod router;
pub mod routes;
pub mod utilities;
pub mod websocket;

pub async fn bootstrap(config: Config, pool: db::Pool) {
    let port = config.port;
    let app = create_router(config, pool);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener to the specified address and port");

    axum::serve(listener, app)
        .await
        .expect("Server encountered an error during execution");
}
