use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

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

        let graphql_schema = Schema::build(
            common::infrastructure::graphql::EcommerceQueryRoot::default(),
            EmptyMutation,
            EmptySubscription,
        )
        .data(services.clone())
        .finish();

        Self {
            router: Router::new().nest(
                "/ecommerce",
                Router::new()
                    .route(
                        "/graphql",
                        get(common::infrastructure::graphql::playground).post(common::infrastructure::graphql::handler),
                    )
                    .with_state(Arc::new(graphql_schema))
                    .nest(
                        "/backoffice",
                        backoffice::infrastructure::HttpController::build(services),
                    ),
            ),
        }
    }
}
