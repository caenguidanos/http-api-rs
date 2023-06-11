use axum::extract::{FromRef, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::Instrument;

use crate::contexts::ecommerce::common::application::usecase::UseCase;
use crate::contexts::ecommerce::{backoffice, common};

pub async fn get_products(
    identity_claims: common::infrastructure::IdentityClaims,
    State(usecase): State<Arc<backoffice::application::usecases::GetProducts>>,
) -> Result<impl IntoResponse, common::domain::Error> {
    identity_claims.check_permission(common::domain::Permissions::EcommerceReadProduct)?;

    let output = usecase
        .exec(())
        .instrument(tracing::debug_span!("Execute use case", name = "GetProducts"))
        .await?;

    Ok(common::infrastructure::JsonResponse::with_status(
        StatusCode::OK,
        output.items,
    ))
}

impl FromRef<common::infrastructure::DependencyContainer> for Arc<backoffice::application::usecases::GetProducts> {
    fn from_ref(input: &common::infrastructure::DependencyContainer) -> Self {
        input.get_products_usecase.clone()
    }
}
