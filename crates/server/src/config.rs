use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self::default();

        let port = env::var("PORT").unwrap_or(config.port.to_string());
        let port = port
            .parse::<u16>()
            .expect("Failed to parse PORT environment variable!");

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

        config.port = port;
        config.database_url = database_url;
        config.jwt_secret = jwt_secret;

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: Default::default(),
            jwt_secret: Default::default(),
            port: 3000,
        }
    }
}
