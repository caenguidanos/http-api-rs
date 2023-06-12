use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::contexts::ecommerce::common;
use crate::libs;

impl IntoResponse for common::domain::Error {
    fn into_response(self) -> Response {
        let mut problem_details: libs::problem_details::ProblemDetails;

        match self {
            Self::InvalidProductId
            | Self::InvalidProductName
            | Self::InvalidProductPrice
            | Self::InvalidProductCurrency
            | Self::ProductAlreadyExists
            | Self::InvalidProductTimeStampRelation => {
                problem_details = libs::problem_details::ProblemDetails::from_400();
                problem_details.set_detail(self);
            }
            Self::InvalidPermission => {
                problem_details = libs::problem_details::ProblemDetails::from_403();
                problem_details.set_detail(self);
            }
            Self::Persistence(reason) => {
                problem_details = libs::problem_details::ProblemDetails::from_503();
                problem_details.set_detail(reason);
            }
        }

        libs::encoding::JsonResponse::with_status(
            StatusCode::from_u16(problem_details.status).unwrap_or(StatusCode::IM_A_TEAPOT),
            problem_details.clone(),
        )
        .into_response()
    }
}
