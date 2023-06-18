use axum::Router;

use crate::libs;

mod backoffice;
mod common;
mod settings;

pub struct HttpContext {
    pub router: Router,
}

impl HttpContext {
    pub async fn new() -> Self {
        let settings = settings::Settings::new();

        let db = libs::postgres::ConnectionManager::new_pool(&settings.database_url, None)
            .await
            .expect("could not initialize postgres connection pool");

        let services = common::infrastructure::DependencyContainer::new(db);

        Self {
            router: Router::new().nest(
                "/ecommerce",
                Router::new().nest(
                    "/backoffice",
                    backoffice::infrastructure::HttpController::build(services),
                ),
            ),
        }
    }
}
