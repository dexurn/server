use server::{bootstrap, config::Config, state::AppState};

#[tokio::main]
async fn main() {
    let config = Config::new();
    let pool = db::create_pool(&config.database_url);
    let app_state = AppState::new();
    bootstrap(config, pool, app_state).await;
}
