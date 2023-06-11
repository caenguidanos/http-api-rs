use std::sync::Arc;

use axum::async_trait;

use super::*;

pub type DynProductRepository<E> = Arc<dyn ProductRepository<Error = E> + Send + Sync + 'static>;

#[async_trait]
pub trait ProductRepository {
    type Error;

    async fn get(&self) -> Result<Vec<Product>, Self::Error>;
    async fn get_by_id(&self, id: &ProductId) -> Result<Option<Product>, Self::Error>;
    async fn save(&self, product: &Product) -> Result<(), Self::Error>;
}
