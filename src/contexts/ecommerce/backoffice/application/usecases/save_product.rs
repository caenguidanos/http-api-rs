use axum::async_trait;
use serde::{Deserialize, Serialize};
use tracing::Instrument;

use crate::contexts::ecommerce::{backoffice, common};

pub struct SaveProduct {
    product_repository: backoffice::domain::product::DynProductRepository<common::domain::Error>,
}

impl SaveProduct {
    pub fn new(product_repository: backoffice::domain::product::DynProductRepository<common::domain::Error>) -> Self {
        Self { product_repository }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveProductInput {
    pub id: String,
    pub name: String,
    pub price: i32,
    pub currency: String,
}

#[async_trait]
impl common::application::usecase::UseCase for SaveProduct {
    type Input = SaveProductInput;
    type Output = ();

    type Error = common::domain::Error;

    async fn exec(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        tracing::debug!("{:?}", input);

        let new_product = backoffice::domain::product::Product::new(input.id, input.name, input.price, input.currency)?;

        self.product_repository
            .save(&new_product)
            .instrument(tracing::info_span!("Invoke ProductRepository.save"))
            .await
    }
}
