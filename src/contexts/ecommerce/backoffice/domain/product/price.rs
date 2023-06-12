use std::fmt::{Display, Formatter};

use crate::contexts::ecommerce::common;

#[derive(Copy, Clone, PartialEq)]
pub struct ProductPrice(i32);

pub const PRODUCT_PRICE_MIN: i32 = 0;
pub const PRODUCT_PRICE_MAX: i32 = 10_000_000 * 100;

impl ProductPrice {
    fn validate(value: impl Into<i32>) -> Result<i32, common::domain::Error> {
        let _e = tracing::debug_span!("Validate ProductPrice").entered();

        let value = value.into();

        if let PRODUCT_PRICE_MIN..=PRODUCT_PRICE_MAX = value {
            return Ok(value);
        }

        Err(common::domain::Error::InvalidProductPrice).inspect_err(|err| tracing::error!("{err}"))
    }

    pub fn to_primitive(self) -> i32 {
        let _e = tracing::debug_span!("Transform ProductPrice to primitive").entered();

        self.0
    }
}

impl Display for ProductPrice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _e = tracing::debug_span!("Display ProductPrice").entered();

        write!(f, "{}", self.0)
    }
}

impl TryFrom<i32> for ProductPrice {
    type Error = common::domain::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let _e = tracing::debug_span!("Try cast ProductPrice from i32").entered();

        Ok(Self(Self::validate(value)?))
    }
}
