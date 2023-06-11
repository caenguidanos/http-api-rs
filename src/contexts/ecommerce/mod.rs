use axum::Router;

use crate::libs;

mod backoffice;
mod common;
mod settings;

pub struct HttpContext;

impl HttpContext {
    pub async fn unsafe_compose() -> Router {
        let settings = settings::Settings::new();

        let db = libs::pg::ConnectionManager::new_pool(&settings.database_url, None)
            .await
            .expect("could not initialize the database connection pool");

        let services = common::infrastructure::DependencyContainer::new(db);

        Router::new().merge(backoffice::infrastructure::HttpController::compose(services))
    }
}
