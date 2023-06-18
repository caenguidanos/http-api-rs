use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use tracing::Instrument;

use crate::contexts::ecommerce::{backoffice, common};

#[axum::debug_handler]
pub async fn handler(
    identity_claims: common::infrastructure::IdentityClaims,
    State(schema): State<Arc<backoffice::infrastructure::graphql::SchemaRoot>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut graphql_request = req.into_inner();
    let graphql_request_query = graphql_request.query.clone();
    graphql_request.data.insert(identity_claims);

    schema
        .execute(graphql_request)
        .instrument(tracing::debug_span!("Executing schema", query = graphql_request_query))
        .await
        .into()
}

#[axum::debug_handler]
pub async fn playground() -> impl IntoResponse {
    let config = GraphQLPlaygroundConfig::new("/ecommerce/backoffice/graphql").with_header("authorization", "Bearer -");

    Html(playground_source(config))
}
