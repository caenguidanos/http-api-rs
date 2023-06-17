use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::{MergedObject, Schema};

use crate::contexts::ecommerce::backoffice;

pub type EcommerceSchema = Schema<EcommerceQueryRoot, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct EcommerceQueryRoot(pub backoffice::infrastructure::graphql::BackofficeQuery);
