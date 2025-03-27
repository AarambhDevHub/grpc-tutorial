#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Self {
        let database_url = "sqlite://todo.db?mode=rwc".to_string();
        Config {
            database_url,
        }
    }
}