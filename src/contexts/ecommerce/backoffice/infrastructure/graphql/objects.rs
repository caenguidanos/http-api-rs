use async_graphql::SimpleObject;

use crate::contexts::ecommerce::backoffice;

#[derive(SimpleObject)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: i32,
    pub currency: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<backoffice::domain::product::Product> for Product {
    fn from(value: backoffice::domain::product::Product) -> Self {
        Self {
            id: value.id.to_primitive(),
            name: value.name.to_primitive(),
            price: value.price.to_primitive(),
            currency: value.currency.to_primitive(),
            created_at: value.created_at.to_primitive(),
            updated_at: value.updated_at.to_primitive(),
        }
    }
}
