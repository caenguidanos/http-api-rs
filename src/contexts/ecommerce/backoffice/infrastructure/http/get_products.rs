use std::sync::Arc;

use axum::extract::{FromRef, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::Instrument;

use crate::contexts::ecommerce::common::application::usecase::UseCase;
use crate::contexts::ecommerce::{backoffice, common};
use crate::libs;

pub async fn get_products(
    identity_claims: common::infrastructure::IdentityClaims,
    State(usecase): State<Arc<backoffice::application::usecases::GetProducts>>,
) -> Result<impl IntoResponse, common::domain::Error> {
    identity_claims.check_permission(common::domain::Permissions::EcommerceReadProduct)?;

    let output = usecase
        .exec(())
        .instrument(tracing::debug_span!("Execute use case", name = "GetProducts"))
        .await?;

    Ok(libs::json::JsonResponse::with_status(StatusCode::OK, output))
}

impl FromRef<common::infrastructure::DependencyContainer> for Arc<backoffice::application::usecases::GetProducts> {
    fn from_ref(input: &common::infrastructure::DependencyContainer) -> Self {
        input.get_products_usecase.clone()
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::get;
    use axum::{http, Router};
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use crate::libs;

    use super::*;

    const PATH: &str = "/ecommerce/product";

    fn router(services: common::infrastructure::DependencyContainer) -> Router {
        Router::new().route(PATH, get(get_products)).with_state(services)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_no_token_when_request_then_return_401() {
        let fixture = common::infrastructure::controller::tests::HttpContextFixture::new().await;

        let response = router(fixture.services)
            .oneshot(Request::builder().uri(PATH).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_no_permissions_when_request_then_return_403() {
        let fixture = common::infrastructure::controller::tests::HttpContextFixture::new().await;

        let response = router(fixture.services)
            .oneshot(
                Request::builder()
                    .uri(PATH)
                    .header(http::header::AUTHORIZATION, fixture.token)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: libs::problem_details::ProblemDetails = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.detail, common::domain::Error::InvalidPermission.to_string(),);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_empty_database_when_request_then_return_200() {
        let mut fixture = common::infrastructure::controller::tests::HttpContextFixture::new().await;
        fixture.with_permissions(&[common::domain::Permissions::EcommerceReadProduct.to_string().as_str()]);

        let response = router(fixture.services)
            .oneshot(
                Request::builder()
                    .uri(PATH)
                    .header(http::header::AUTHORIZATION, fixture.token)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, json!([]));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn given_products_on_database_when_request_then_return_200() {
        let mut fixture = common::infrastructure::controller::tests::HttpContextFixture::new().await;
        fixture.with_permissions(&[common::domain::Permissions::EcommerceReadProduct.to_string().as_str()]);

        let product_1 = backoffice::domain::product::fixture::ProductBuilder::default();
        product_1.save(&fixture.services.product_repository).await;

        let product_2 = backoffice::domain::product::fixture::ProductBuilder::default();
        product_2.save(&fixture.services.product_repository).await;

        let response = router(fixture.services)
            .oneshot(
                Request::builder()
                    .uri(PATH)
                    .header(http::header::AUTHORIZATION, fixture.token)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let body = body.as_array().unwrap();

        assert_eq!(body.len(), 2);
    }
}
