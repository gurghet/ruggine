use axum::{
    response::{Html, IntoResponse},
    http::StatusCode,
};
use std::fs;

// Constants

pub async fn root_handler() -> impl IntoResponse {
    match fs::read_to_string("static/index.html") {
        Ok(content) => Html(content).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to load static content"
        ).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
        routing::get,
    };
    use tower::ServiceExt;
    use axum::http::header::CONTENT_TYPE;

    #[tokio::test]
    async fn test_root_handler() {
        let app = Router::new()
            .route("/", get(root_handler));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8",
            "Root endpoint should return HTML content type"
        );

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let content = String::from_utf8(body.to_vec()).unwrap();
        assert!(content.starts_with("<!DOCTYPE html>"));
    }
}