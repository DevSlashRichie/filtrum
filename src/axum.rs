use std::str::FromStr;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};

use crate::{common::WithFilterId, errors::FilterParseError, query_filter::FromQueryFilter};

pub struct FilterRejection(pub FilterParseError);

impl IntoResponse for FilterRejection {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.0.to_string()).into_response()
    }
}

impl<T, S> FromRequestParts<S> for FromQueryFilter<T>
where
    T: FromStr<Err = FilterParseError> + WithFilterId + Default + Send + Sync,
    S: Send + Sync,
{
    type Rejection = FilterRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or("");
        Self::from_str(query).map_err(FilterRejection)
    }
}