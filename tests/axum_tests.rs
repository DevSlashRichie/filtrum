#[cfg(feature = "axum")]
mod tests {
    use axum::{routing::get, Router, http::StatusCode};
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;
    use filtrum::query_filter::FromQueryFilter;
    use filtrum::common::WithFilterId;
    use filtrum::equal_filter::EqualFilter;
    use filtrum::errors::FilterParseError;
    use std::str::FromStr;

    #[derive(Default, Debug)]
    struct MyFilter {
        age: EqualFilter<i32>,
    }

    impl WithFilterId for MyFilter {
        fn filter_id() -> Option<&'static str> { None }
    }

    impl FromStr for MyFilter {
        type Err = FilterParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
             Ok(MyFilter { age: EqualFilter::from_str("age", s)? })
        }
    }

    async fn handler(filter: FromQueryFilter<MyFilter>) -> String {
        format!("age: {:?}", filter.inner.age.into_inner())
    }

    #[tokio::test]
    async fn test_axum_extractor() {
        let app = Router::new().route("/", get(handler));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/?age=25")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body_bytes[..], b"age: Some(25)");
    }

    #[tokio::test]
    async fn test_axum_extractor_error() {
        let app = Router::new().route("/", get(handler));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/?age=not_a_number")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
