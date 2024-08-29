use server::{bootstrap, config::Config};

#[tokio::main]
async fn main() {
    let config = Config::new();

    bootstrap(config).await;
}
