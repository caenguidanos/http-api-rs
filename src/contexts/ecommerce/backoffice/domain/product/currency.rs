use std::fmt::{Display, Formatter};

use crate::contexts::ecommerce::common;

#[derive(Copy, Clone, PartialEq)]
pub enum ProductCurrency {
    Eur,
    Usd,
}

impl ProductCurrency {
    pub fn to_primitive(self) -> String {
        let _e = tracing::debug_span!("Transform ProductCurrency to primitive").entered();

        self.to_string()
    }
}

impl Display for ProductCurrency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _e = tracing::debug_span!("Display ProductCurrency").entered();

        match self {
            Self::Eur => write!(f, "EUR"),
            Self::Usd => write!(f, "USD"),
        }
    }
}

impl TryFrom<&str> for ProductCurrency {
    type Error = common::domain::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let _e = tracing::debug_span!("Try cast ProductCurrency from &str").entered();

        match value {
            "EUR" => Ok(ProductCurrency::Eur),
            "USD" => Ok(ProductCurrency::Usd),
            _ => Err(common::domain::Error::InvalidProductCurrency).inspect_err(|err| tracing::error!("{err}")),
        }
    }
}

impl TryFrom<String> for ProductCurrency {
    type Error = common::domain::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let _e = tracing::debug_span!("Try cast ProductCurrency from String").entered();

        Self::try_from(value.as_str())
    }
}
