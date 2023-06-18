pub use currency::*;
pub use id::*;
pub use name::*;
pub use price::*;
pub use repository::*;
pub use timestamp::*;

use crate::contexts::ecommerce::common;

mod currency;
mod id;
mod name;
mod price;
mod repository;
mod timestamp;

pub struct Product {
    pub id: ProductId,
    pub name: ProductName,
    pub price: ProductPrice,
    pub currency: ProductCurrency,
    pub created_at: ProductTimeStamp,
    pub updated_at: ProductTimeStamp,
}

impl Product {
    pub fn new(id: String, name: String, price: i32, currency: String) -> Result<Self, common::domain::Error> {
        let now = ProductTimeStamp::default();

        let product = Self {
            id: ProductId::try_from(id)?,
            name: ProductName::try_from(name)?,
            price: ProductPrice::try_from(price)?,
            currency: ProductCurrency::try_from(currency)?,
            updated_at: now,
            created_at: now,
        };

        product.validate()?;

        Ok(product)
    }

    pub fn validate(&self) -> Result<(), common::domain::Error> {
        let _e = tracing::debug_span!("Validate Product").entered();

        if self.created_at > self.updated_at {
            return Err(common::domain::Error::InvalidProductTimeStampRelation)
                .inspect_err(|err| tracing::error!("{err}"));
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod fixture {
    use crate::contexts::ecommerce::common;
    use crate::libs;

    use super::*;

    pub struct ProductBuilder {
        pub id: ProductId,
        pub name: ProductName,
        pub price: ProductPrice,
        pub currency: ProductCurrency,
        pub created_at: ProductTimeStamp,
        pub updated_at: ProductTimeStamp,
    }

    impl Default for ProductBuilder {
        fn default() -> Self {
            let now = ProductTimeStamp::default();

            let random_name = libs::random::generate_alphanumeric_string(None);
            let random_price = libs::random::generate_int_from_range(Some(PRODUCT_PRICE_MIN), Some(PRODUCT_PRICE_MAX));

            Self {
                id: ProductId::default(),
                name: ProductName::try_from(random_name).unwrap(),
                price: ProductPrice::try_from(random_price).unwrap(),
                currency: ProductCurrency::Eur,
                updated_at: now,
                created_at: now,
            }
        }
    }

    impl ProductBuilder {
        pub fn to_entity(&self) -> Product {
            let entity = Product {
                id: self.id,
                name: self.name.clone(),
                price: self.price,
                currency: self.currency,
                updated_at: self.updated_at,
                created_at: self.created_at,
            };

            entity.validate().unwrap();
            entity
        }

        pub async fn save(&self, repository: &DynProductRepository<common::domain::Error>) {
            repository.save(&self.to_entity()).await.unwrap()
        }
    }
}
