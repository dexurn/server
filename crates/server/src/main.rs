use server::{bootstrap, config::Config};

#[tokio::main]
async fn main() {
    let config = Config::new();

    let pool = db::create_pool(&config.database_url);

    bootstrap(config, pool).await;
}
