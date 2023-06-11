pub struct Settings {
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL"),
        }
    }
}
