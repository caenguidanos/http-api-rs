use std::sync::Arc;

use axum::extract;
use axum::extract::{FromRef, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::Instrument;

use crate::contexts::ecommerce::common::application::usecase::UseCase;
use crate::contexts::ecommerce::{backoffice, common};

#[axum::debug_handler]
pub async fn save_product(
    identity_claims: common::infrastructure::IdentityClaims,
    State(usecase): State<Arc<backoffice::application::usecases::SaveProduct>>,
    extract::Json(body): extract::Json<backoffice::application::usecases::SaveProductInput>,
) -> Result<impl IntoResponse, common::domain::Error> {
    identity_claims.check_permission(common::domain::Permissions::EcommerceBackofficeProductCreate)?;

    usecase
        .exec(body)
        .instrument(tracing::debug_span!("Execute use case", name = "SaveProduct"))
        .await?;

    Ok(StatusCode::ACCEPTED)
}

impl FromRef<common::infrastructure::DependencyContainer> for Arc<backoffice::application::usecases::SaveProduct> {
    fn from_ref(input: &common::infrastructure::DependencyContainer) -> Self {
        input.save_product_usecase.clone()
    }
}
