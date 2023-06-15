use axum::routing::get;
use axum::Router;

use crate::contexts::ecommerce::{backoffice, common};

pub struct HttpController;

impl HttpController {
    pub fn build(services: common::infrastructure::DependencyContainer) -> Router {
        Router::new()
            .nest(
                "/product",
                Router::new().route(
                    "/",
                    get(backoffice::infrastructure::get_products).put(backoffice::infrastructure::save_product),
                ),
            )
            .with_state(services)
    }
}
