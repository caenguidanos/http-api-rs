pub struct Settings {
    pub gql_playground_enabled: bool,
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Self {
        let gql_playground_enabled: bool = std::env::var("GRAPHQL_PLAYGROUND_ENABLED")
            .unwrap_or(String::from("false"))
            .parse::<bool>()
            .unwrap_or(false);

        Self {
            gql_playground_enabled,
            database_url: std::env::var("ECOMMERCE__DATABASE_URL").expect("ECOMMERCE__DATABASE_URL"),
        }
    }
}
