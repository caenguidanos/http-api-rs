use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

use crate::contexts::ecommerce::backoffice;

impl Serialize for backoffice::domain::product::Product {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let _e = tracing::debug_span!("Serialize Product").entered();

        let mut state = serializer.serialize_struct("Product", 6)?;

        state.serialize_field("id", &self.id.to_primitive())?;
        state.serialize_field("name", &self.name.to_primitive())?;
        state.serialize_field("price", &self.price.to_primitive())?;
        state.serialize_field("currency", &self.currency.to_primitive())?;
        state.serialize_field("updated_at", &self.updated_at.to_primitive())?;
        state.serialize_field("created_at", &self.created_at.to_primitive())?;

        state.end()
    }
}

impl FromRow<'_, PgRow> for backoffice::domain::product::Product {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        let _e = tracing::debug_span!("Cast Product from PgRow").entered();

        let id: uuid::Uuid = row.try_get(0).inspect_err(|err| tracing::error!("{err}"))?;
        let id = backoffice::domain::product::ProductId::from(id);

        let name: String = row.try_get(1).inspect_err(|err| tracing::error!("{err}"))?;
        let name = backoffice::domain::product::ProductName::try_from(name).map_err(|_| Error::TypeNotFound {
            type_name: String::from("ProductName"),
        })?;

        let price: i32 = row.try_get(2).inspect_err(|err| tracing::error!("{err}"))?;
        let price = backoffice::domain::product::ProductPrice::try_from(price).map_err(|_| Error::TypeNotFound {
            type_name: String::from("ProductPrice"),
        })?;

        let currency: String = row.try_get(3).inspect_err(|err| tracing::error!("{err}"))?;
        let currency =
            backoffice::domain::product::ProductCurrency::try_from(currency).map_err(|_| Error::TypeNotFound {
                type_name: String::from("ProductCurrency"),
            })?;

        let created_at: chrono::DateTime<chrono::offset::Utc> =
            row.try_get(4).inspect_err(|err| tracing::error!("{err}"))?;
        let created_at = backoffice::domain::product::ProductTimeStamp::from(created_at);

        let updated_at: chrono::DateTime<chrono::offset::Utc> =
            row.try_get(5).inspect_err(|err| tracing::error!("{err}"))?;
        let updated_at = backoffice::domain::product::ProductTimeStamp::from(updated_at);

        Ok(backoffice::domain::product::Product {
            id,
            name,
            price,
            currency,
            created_at,
            updated_at,
        })
    }
}
