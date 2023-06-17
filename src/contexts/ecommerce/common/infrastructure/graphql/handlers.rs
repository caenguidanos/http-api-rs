use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use std::sync::Arc;
use tracing::Instrument;

use crate::contexts::ecommerce::common;

#[axum::debug_handler]
pub async fn handler(
    _identity_claims: common::infrastructure::IdentityClaims,
    State(schema): State<Arc<common::infrastructure::graphql::EcommerceSchema>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let graphql_request = req.into_inner();
    let graphql_request_query = graphql_request.query.clone();

    schema
        .execute(graphql_request)
        .instrument(tracing::debug_span!("Executing schema", query = graphql_request_query))
        .await
        .into()
}

#[axum::debug_handler]
pub async fn playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/ecommerce/graphql").subscription_endpoint("/ecommerce/graphql/ws"),
    ))
}
