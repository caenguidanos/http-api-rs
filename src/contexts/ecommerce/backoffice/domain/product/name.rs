use std::fmt::{Display, Formatter};

use crate::contexts::ecommerce::common;

#[derive(Clone, PartialEq)]
pub struct ProductName(String);

impl ProductName {
    fn validate(value: impl Into<String>) -> Result<String, common::domain::Error> {
        let _e = tracing::debug_span!("Validate ProductName").entered();

        let value = value.into();

        const NAME_MIN_LENGTH: usize = 1;
        const NAME_MAX_LENGTH: usize = 256;

        if let NAME_MIN_LENGTH..=NAME_MAX_LENGTH = value.len() {
            return Ok(value);
        }

        Err(common::domain::Error::InvalidProductName).inspect_err(|err| tracing::error!("{err}"))
    }

    pub fn to_primitive(&self) -> String {
        let _e = tracing::debug_span!("Transform ProductName to primitive").entered();

        self.0.clone()
    }
}

impl Display for ProductName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _e = tracing::debug_span!("Display ProductName").entered();

        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for ProductName {
    type Error = common::domain::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let _e = tracing::debug_span!("Try cast ProductName from &str").entered();

        Ok(Self(Self::validate(value)?))
    }
}

impl TryFrom<String> for ProductName {
    type Error = common::domain::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let _e = tracing::debug_span!("Try cast ProductName from String").entered();

        Self::try_from(value.as_str())
    }
}
