use std::fmt::Display;

use axum::async_trait;

#[async_trait]
pub trait UseCase {
    type Input;
    type Output;
    type Error: Display;

    async fn exec(&self, request: Self::Input) -> Result<Self::Output, Self::Error>;
}
