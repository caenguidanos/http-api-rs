use axum::async_trait;

use crate::contexts::ecommerce::{backoffice, common};
use crate::libs;

pub struct PostgresProductRepository {
    db: libs::pg::ConnectionPool,
}

impl PostgresProductRepository {
    pub fn new(db: libs::pg::ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl backoffice::domain::product::ProductRepository for PostgresProductRepository {
    type Error = common::domain::Error;

    async fn get(&self) -> Result<Vec<backoffice::domain::product::Product>, Self::Error> {
        static SQL: &str = r#"
            SELECT *
            FROM product
            LIMIT 50
        "#;

        sqlx::query_as(SQL)
            .fetch_all(&self.db)
            .await
            .inspect_err(|err| tracing::error!("{err}"))
            .map_err(|err| common::domain::Error::Persistence(err.to_string()))
    }

    async fn get_by_id(
        &self,
        id: &backoffice::domain::product::ProductId,
    ) -> Result<Option<backoffice::domain::product::Product>, Self::Error> {
        static SQL: &str = r#"
            SELECT *
            FROM product
            WHERE id = $1
        "#;

        sqlx::query_as(SQL)
            .bind(id.to_primitive())
            .fetch_optional(&self.db)
            .await
            .inspect_err(|err| tracing::error!("{err}"))
            .map_err(|err| common::domain::Error::Persistence(err.to_string()))
    }

    async fn save(&self, product: &backoffice::domain::product::Product) -> Result<(), Self::Error> {
        static SQL: &str = r#"
            INSERT INTO product (id, name, price, currency)
            VALUES ($1, $2, $3, $4)
        "#;

        sqlx::query(SQL)
            .bind(product.id.to_primitive())
            .bind(product.name.to_primitive())
            .bind(product.price.to_primitive())
            .bind(product.currency.to_string())
            .execute(&self.db)
            .await
            .inspect_err(|err| tracing::error!("{err}"))
            .map_err(|error| {
                if let Some(error) = error.as_database_error() {
                    if let Some(code) = error.code() {
                        if code == libs::pg::errcodes::Codes::UniqueViolation.to_string() {
                            return common::domain::Error::ProductAlreadyExists;
                        }
                    }
                }

                common::domain::Error::Persistence(error.to_string())
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::contexts::ecommerce::backoffice;
    use crate::libs;

    use super::*;

    async fn fixture() -> backoffice::domain::product::DynProductRepository<common::domain::Error> {
        let database = libs::pg::tests::UnsafePostgresDatabase::new().await;

        Arc::new(PostgresProductRepository::new(database.pool))
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_empty_database_when_get_then_return_empty_vec() {
        let repository = fixture().await;

        assert!(repository.get().await.unwrap().is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_products_on_database_when_get_then_return_fulfilled_vec() {
        let repository = fixture().await;

        let p1 = backoffice::domain::product::tests::UnsafeProductBuilder::default();
        let p2 = backoffice::domain::product::tests::UnsafeProductBuilder::default();
        let p3 = backoffice::domain::product::tests::UnsafeProductBuilder::default();
        let p4 = backoffice::domain::product::tests::UnsafeProductBuilder::default();
        let p5 = backoffice::domain::product::tests::UnsafeProductBuilder::default();

        tokio::join!(
            p1.save(&repository),
            p2.save(&repository),
            p3.save(&repository),
            p4.save(&repository),
            p5.save(&repository)
        );

        assert_eq!(repository.get().await.unwrap().len(), 5);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_empty_database_when_get_by_id_then_return_none() {
        let repository = fixture().await;

        let id = backoffice::domain::product::ProductId::default();

        assert!(repository.get_by_id(&id).await.unwrap().is_none());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_products_on_database_when_get_by_id_then_return_some() {
        let repository = fixture().await;

        let product = backoffice::domain::product::tests::UnsafeProductBuilder::default();
        product.save(&repository).await;

        assert!(repository.get_by_id(&product.id).await.unwrap().is_some());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_empty_database_when_save_then_return_ok() {
        let repository = fixture().await;

        let product = backoffice::domain::product::tests::UnsafeProductBuilder::default();

        assert!(repository.save(&product.to_entity()).await.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_products_on_database_when_save_with_same_id_then_return_err() {
        let repository = fixture().await;

        let product = backoffice::domain::product::tests::UnsafeProductBuilder::default();

        assert!(repository.save(&product.to_entity()).await.is_ok());

        matches!(
            repository.save(&product.to_entity()).await.err().unwrap(),
            common::domain::Error::ProductAlreadyExists
        );
    }
}
