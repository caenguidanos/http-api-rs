use std::fmt::{Display, Formatter};

use crate::contexts::ecommerce::common;

#[derive(Copy, Clone, PartialEq)]
pub struct ProductId(uuid::Uuid);

impl ProductId {
    fn validate(value: impl Into<String>) -> Result<uuid::Uuid, common::domain::Error> {
        let _e = tracing::debug_span!("Validate ProductId").entered();

        uuid::Uuid::parse_str(&value.into())
            .inspect_err(|err| tracing::error!("{err}"))
            .map_err(|_| common::domain::Error::InvalidProductId)
    }

    pub fn to_uuid(self) -> uuid::Uuid {
        let _e = tracing::debug_span!("Transform ProductId to uuid").entered();

        self.0
    }

    pub fn to_primitive(self) -> String {
        let _e = tracing::debug_span!("Transform ProductId to primitive").entered();

        self.0.to_string()
    }
}

impl Default for ProductId {
    fn default() -> Self {
        let _e = tracing::debug_span!("New ProductId").entered();

        Self(uuid::Uuid::new_v4())
    }
}

impl Display for ProductId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _e = tracing::debug_span!("Display ProductId").entered();

        write!(f, "{}", self.0)
    }
}

impl From<uuid::Uuid> for ProductId {
    fn from(value: uuid::Uuid) -> Self {
        let _e = tracing::debug_span!("Cast ProductId from uuid::Uuid").entered();

        Self(value)
    }
}

impl TryFrom<&str> for ProductId {
    type Error = common::domain::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let _e = tracing::debug_span!("Try cast ProductId from &str").entered();

        Ok(Self(Self::validate(value)?))
    }
}

impl TryFrom<String> for ProductId {
    type Error = common::domain::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let _e = tracing::debug_span!("Try cast ProductId from String").entered();

        Self::try_from(value.as_str())
    }
}
