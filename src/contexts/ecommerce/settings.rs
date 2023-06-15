pub struct Settings {
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            database_url: std::env::var("ECOMMERCE__DATABASE_URL").expect("ECOMMERCE__DATABASE_URL"),
        }
    }
}
