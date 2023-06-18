use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use axum::routing::get;
use axum::Router;

use crate::contexts::ecommerce::{backoffice, common};

pub struct HttpController;

impl HttpController {
    pub fn build(services: common::infrastructure::DependencyContainer) -> Router {
        let cloned_services = services.clone();

        let graphql_schema = Arc::new(
            Schema::build(
                backoffice::infrastructure::graphql::QueryRoot,
                backoffice::infrastructure::graphql::MutationRoot,
                EmptySubscription,
            )
            .data(cloned_services)
            .finish(),
        );

        Router::new()
            .route(
                "/graphql",
                get(backoffice::infrastructure::graphql::playground)
                    .post(backoffice::infrastructure::graphql::handler)
                    .with_state(graphql_schema),
            )
            .nest(
                "/product",
                Router::new().route(
                    "/",
                    get(backoffice::infrastructure::http::get_products)
                        .put(backoffice::infrastructure::http::save_product),
                ),
            )
            .with_state(services)
    }
}
