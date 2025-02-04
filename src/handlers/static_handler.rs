use axum::{
    response::{Html, IntoResponse},
    http::{StatusCode, header::CONTENT_TYPE},
};
use std::fs;
use percent_encoding::percent_decode_str;
use std::path::Path;

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
    // If the path doesn't start with "images/", return 404
    if !file_path.starts_with("images/") {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }

    // Decode the URL-encoded path
    let decoded_path = percent_decode_str(file_path)
        .decode_utf8_lossy()
        .into_owned();

    let full_path = format!("static/{}", decoded_path);
    println!("Attempting to read file at: {}", full_path);

    // Handle file reading and response
    match fs::read(full_path) {
        Ok(content) => {
            let content_type = match Path::new(file_path).extension().and_then(|e| e.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("svg") => "image/svg+xml",
                Some("html") => "text/html; charset=utf-8",
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                _ => "application/octet-stream",
            };

            ([(CONTENT_TYPE, content_type)], content).into_response()
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            (StatusCode::NOT_FOUND, "File not found").into_response()
        }
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
    async fn test_static_file_handler_in_subdirectory() {
        let app = Router::new()
            .route("/static/*file", get(|path: axum::extract::Path<String>| async move {
                static_file_handler(&path.0).await
            }));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/static/images/cce-logo.png")
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

    #[tokio::test]
    async fn test_static_dir_returns_404_except_images() {
        let app = Router::new()
            .route("/static/*file", get(|path: axum::extract::Path<String>| async move {
                static_file_handler(&path.0).await
            }));

        // Test various paths that should return 404
        let test_cases = vec![
            "/static/something.txt",           // Random file
            "/static/index.html",             // Even though index.html exists
            "/static/css/styles.css",         // CSS files
            "/static/js/script.js",           // JavaScript files
            "/static/assets/favicon.ico",     // Other assets
        ];

        for path in test_cases {
            let response = app.clone()
                .oneshot(
                    Request::builder()
                        .uri(path)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::NOT_FOUND, "Path {} should return 404", path);
            let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
            assert_eq!(&body[..], b"Not Found", "Path {} should return 'Not Found'", path);
        }

        // Test that /static/images/nonexistent.png returns 404 with a different message
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .uri("/static/images/nonexistent.png")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(&body[..], b"File not found", "Missing image should return 'File not found'");

        // Test that /static/images/cce-logo.png returns 200
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/static/images/cce-logo.png")
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

    #[tokio::test]
    async fn test_static_index_returns_404() {
        let app = Router::new()
            .route("/static/*file", get(|path: axum::extract::Path<String>| async move {
                static_file_handler(&path.0).await
            }));

        // Test that /static/index.html returns 404 even though the file exists
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/static/index.html")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(&body[..], b"Not Found");
    }
}