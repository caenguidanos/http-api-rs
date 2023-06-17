use async_graphql::{Context, Object};
use tracing::Instrument;

use crate::contexts::ecommerce::common;
use crate::contexts::ecommerce::common::application::usecase::UseCase;

mod product;

#[derive(Default)]
pub struct BackofficeQuery;

#[Object]
impl BackofficeQuery {
    pub async fn products<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Vec<product::Product>> {
        let services = ctx.data::<common::infrastructure::DependencyContainer>()?;

        let products = services
            .get_products_usecase
            .exec(())
            .instrument(tracing::debug_span!("Execute use case", name = "GetProducts"))
            .await?;

        Ok(products.into_iter().map(product::Product::from).collect())
    }
}
