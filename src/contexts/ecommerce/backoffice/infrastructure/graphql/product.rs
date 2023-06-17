use async_graphql::SimpleObject;

use crate::contexts::ecommerce::backoffice;

#[derive(SimpleObject)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: i32,
    pub currency: String,
    pub __created_at__: String,
    pub __updated_at__: String,
}

impl From<backoffice::domain::product::Product> for Product {
    fn from(value: backoffice::domain::product::Product) -> Self {
        Self {
            id: value.id.to_primitive(),
            name: value.name.to_primitive(),
            price: value.price.to_primitive(),
            currency: value.currency.to_primitive(),
            __created_at__: value.__created_at__.to_primitive(),
            __updated_at__: value.__updated_at__.to_primitive(),
        }
    }
}
