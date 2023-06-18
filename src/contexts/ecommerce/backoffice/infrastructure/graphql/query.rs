use async_graphql::{Context, EmptySubscription};
use async_graphql::{Object, Schema};
use tracing::Instrument;

use crate::contexts::ecommerce::common::application::usecase::UseCase;
use crate::contexts::ecommerce::{backoffice, common};

pub type SchemaRoot = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn products<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> async_graphql::Result<Vec<backoffice::infrastructure::graphql::Product>> {
        let claims = ctx.data::<common::infrastructure::IdentityClaims>()?;
        claims.check_permission(common::domain::Permissions::EcommerceBackofficeProductRead)?;

        let services = ctx.data::<common::infrastructure::DependencyContainer>()?;

        let products = services
            .get_products_usecase
            .exec(())
            .instrument(tracing::debug_span!("Execute use case", name = "GetProducts"))
            .await?;

        Ok(products
            .into_iter()
            .map(backoffice::infrastructure::graphql::Product::from)
            .collect())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn save_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: String,
        name: String,
        price: i32,
        currency: String,
    ) -> async_graphql::Result<bool> {
        let claims = ctx.data::<common::infrastructure::IdentityClaims>()?;
        claims.check_permission(common::domain::Permissions::EcommerceBackofficeProductCreate)?;

        let services = ctx.data::<common::infrastructure::DependencyContainer>()?;

        services
            .save_product_usecase
            .exec(backoffice::application::usecases::SaveProductInput {
                id,
                name,
                price,
                currency,
            })
            .instrument(tracing::debug_span!("Execute use case", name = "SaveProduct"))
            .await?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_graphql::{EmptySubscription, Schema};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::post;
    use axum::{http, Router};
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use crate::contexts::ecommerce::common;

    use super::*;

    const PATH: &str = "/graphql";

    fn router(services: common::infrastructure::DependencyContainer) -> Router {
        let schema = Arc::new(
            Schema::build(QueryRoot, MutationRoot, EmptySubscription)
                .data(services)
                .finish(),
        );

        Router::new().route(
            PATH,
            post(backoffice::infrastructure::graphql::handler).with_state(schema),
        )
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_empty_database_when_request_then_return_200() {
        let mut fixture = common::infrastructure::controller::fixture::HttpContextFixture::new().await;
        fixture.with_permissions(&[common::domain::Permissions::EcommerceBackofficeProductRead
            .to_string()
            .as_str()]);

        let body = json!({
            "operationName": "Query",
            "variables": {},
            "query": "query Query { products { id }}"
        });

        let response = router(fixture.services)
            .oneshot(
                Request::builder()
                    .uri(PATH)
                    .method("POST")
                    .header(http::header::AUTHORIZATION, fixture.token)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string())
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            json!({
              "data": {
                "products": []
              }
            })
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_products_on_database_when_request_then_return_200() {
        let mut fixture = common::infrastructure::controller::fixture::HttpContextFixture::new().await;
        fixture.with_permissions(&[common::domain::Permissions::EcommerceBackofficeProductRead
            .to_string()
            .as_str()]);

        let product_1 = backoffice::domain::product::fixture::ProductBuilder::default();
        product_1.save(&fixture.services.product_repository).await;

        let product_2 = backoffice::domain::product::fixture::ProductBuilder::default();
        product_2.save(&fixture.services.product_repository).await;

        let body = json!({
            "operationName": "Query",
            "variables": {},
            "query": "query Query { products { id }}"
        });

        let response = router(fixture.services)
            .oneshot(
                Request::builder()
                    .uri(PATH)
                    .method("POST")
                    .header(http::header::AUTHORIZATION, fixture.token)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string())
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            json!({
              "data": {
                "products": [
                        {
                            "id": product_1.id.to_primitive()
                        },
                        {
                            "id": product_2.id.to_primitive()
                        }
                    ]
              }
            })
        );
    }
}
