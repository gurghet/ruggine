use axum::{
    response::{Html, IntoResponse},
    http::{StatusCode, header::CONTENT_TYPE},
};
use std::fs;
use percent_encoding::percent_decode_str;

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

pub async fn static_file_handler(file_path: &str) -> impl IntoResponse {
    // Decode the URL-encoded path
    let decoded_path = percent_decode_str(file_path)
        .decode_utf8_lossy()
        .into_owned();

    match fs::read(format!("static/{}", decoded_path)) {
        Ok(content) => (
            StatusCode::OK,
            [(CONTENT_TYPE, "image/png")],
            content
        ).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            "File not found"
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

    #[tokio::test]
    async fn test_static_file_handler() {
        let app = Router::new()
            .route("/static/*file", get(|path: axum::extract::Path<String>| async move {
                static_file_handler(&path.0).await
            }));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/static/CodeCraft%20Engineering%20logo.png")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "image/png"
        );
    }
}