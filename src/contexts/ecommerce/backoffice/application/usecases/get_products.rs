use axum::async_trait;
use tracing::Instrument;

use crate::contexts::ecommerce::{backoffice, common};

pub struct GetProducts {
    product_repository: backoffice::domain::product::DynProductRepository<common::domain::Error>,
}

impl GetProducts {
    pub fn new(product_repository: backoffice::domain::product::DynProductRepository<common::domain::Error>) -> Self {
        Self { product_repository }
    }
}

#[async_trait]
impl common::application::usecase::UseCase for GetProducts {
    type Input = ();
    type Output = Vec<backoffice::domain::product::Product>;

    type Error = common::domain::Error;

    async fn exec(&self, _: Self::Input) -> Result<Self::Output, Self::Error> {
        self.product_repository
            .get()
            .instrument(tracing::info_span!("Invoke ProductRepository.get"))
            .await
    }
}
