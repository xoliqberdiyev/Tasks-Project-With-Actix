pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
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
        }
    }
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
