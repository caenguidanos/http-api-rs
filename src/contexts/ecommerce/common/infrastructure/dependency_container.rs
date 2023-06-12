use std::sync::Arc;

use crate::contexts::ecommerce::{backoffice, common};
use crate::libs;

#[derive(Clone)]
pub struct DependencyContainer {
    pub product_repository: backoffice::domain::product::DynProductRepository<common::domain::Error>,

    pub get_products_usecase: Arc<backoffice::application::usecases::GetProducts>,
    pub save_product_usecase: Arc<backoffice::application::usecases::SaveProduct>,
}

impl DependencyContainer {
    pub fn new(db: libs::postgres::ConnectionPool) -> Self {
        let product_repository = Arc::new(backoffice::infrastructure::PostgresProductRepository::new(db));

        Self {
            product_repository: product_repository.clone(),

            get_products_usecase: Arc::new(backoffice::application::usecases::GetProducts::new(
                product_repository.clone(),
            )),
            save_product_usecase: Arc::new(backoffice::application::usecases::SaveProduct::new(product_repository)),
        }
    }
}
