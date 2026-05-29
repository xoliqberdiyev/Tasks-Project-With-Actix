#[derive(Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub secret_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .unwrap(),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://database/tasks.db".to_string()), // for test development
            secret_key: std::env::var("SECRET_KEY")
                .expect("SECRET_KEY is not set in .env file"),
        }
    }
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    pub fn secret_key_bytes(&self) -> &[u8] {
        self.secret_key.as_bytes()
    }
}
