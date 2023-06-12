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
    pub __created_at__: ProductTimeStamp,
    pub __updated_at__: ProductTimeStamp,
}

impl Product {
    pub fn new(id: String, name: String, price: i32, currency: String) -> Result<Self, common::domain::Error> {
        let now = ProductTimeStamp::default();

        let product = Self {
            id: ProductId::try_from(id)?,
            name: ProductName::try_from(name)?,
            price: ProductPrice::try_from(price)?,
            currency: ProductCurrency::try_from(currency)?,
            __updated_at__: now,
            __created_at__: now,
        };

        product.validate()?;

        Ok(product)
    }

    pub fn validate(&self) -> Result<(), common::domain::Error> {
        let _e = tracing::debug_span!("Validate ProductPrice").entered();

        if self.__created_at__ > self.__updated_at__ {
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
        pub __created_at__: ProductTimeStamp,
        pub __updated_at__: ProductTimeStamp,
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
                __updated_at__: now,
                __created_at__: now,
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
                __updated_at__: self.__updated_at__,
                __created_at__: self.__created_at__,
            };

            entity.validate().unwrap();
            entity
        }

        pub async fn save(&self, repository: &DynProductRepository<common::domain::Error>) {
            repository.save(&self.to_entity()).await.unwrap()
        }
    }
}
