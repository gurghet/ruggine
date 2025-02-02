use axum::{
    routing::get,
    Router,
    response::{Redirect, Html, Json},
    extract::Path,
    http::header::CONTENT_TYPE,
};
use axum::serve;
use std::net::SocketAddr;
use std::collections::HashMap;

const STATIC_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>Hi</title>
</head>
<body>
</body>
</html>"#;

async fn root_handler() -> Html<&'static str> {
    Html(STATIC_HTML)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct VersionResponse {
    version: String,
}

#[tokio::main]
async fn main() {
    // Create static URL mappings
    let mut url_map = HashMap::new();
    url_map.insert("B5Z".to_string(), "https://codecraft.engineering".to_string());

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/url/:code", get(url_redirect_handler))
        .route("/version", get(version_handler));

    // Run our app
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn url_redirect_handler(Path(code): Path<String>) -> Redirect {
    match code.as_str() {
        "B5Z" => Redirect::temporary("https://codecraft.engineering"),
        _ => Redirect::temporary("/"),
    }
}

async fn version_handler() -> Json<VersionResponse> {
    let version = std::env::var("APP_VERSION").unwrap_or_else(|_| "unknown".to_string());
    Json(VersionResponse { version })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        http::{Request, StatusCode},
        Router,
        body::to_bytes,
    };
    use tower::ServiceExt;
    use serde_json::json;

    #[tokio::test]
    async fn test_valid_redirect() {
        let app = Router::new()
            .route("/url/:code", get(url_redirect_handler));

        let response = app
            .oneshot(Request::builder().uri("/url/B5Z").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(
            response.headers().get("location").unwrap(),
            "https://codecraft.engineering"
        );
    }

    #[tokio::test]
    async fn test_invalid_redirect() {
        let app = Router::new().route("/url/:code", get(url_redirect_handler));

        let response = app
            .oneshot(Request::builder().uri("/url/INVALID").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(
            response.headers().get("location").unwrap(),
            "/"
        );
    }

    #[tokio::test]
    async fn test_empty_code() {
        let app = Router::new().route("/url/:code", get(url_redirect_handler));

        let response = app
            .oneshot(Request::builder().uri("/url/").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_wrong_path() {
        let app = Router::new().route("/url/:code", get(url_redirect_handler));

        let response = app
            .oneshot(Request::builder().uri("/wrong/B5Z").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_root_handler() {
        let app = Router::new()
            .route("/", get(root_handler));

        let response = app
            .oneshot(Request::builder().uri("/").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8",
            "Root endpoint should return HTML content type"
        );
        
        let body = to_bytes(response.into_body(), 1024 * 32).await.unwrap();
        assert_eq!(
            String::from_utf8(body.to_vec()).unwrap(),
            STATIC_HTML
        );
    }

    #[tokio::test]
    async fn test_version_endpoint() {
        std::env::set_var("APP_VERSION", "v1.2.3");
        
        let app = Router::new()
            .route("/version", get(version_handler));

        let response = app
            .oneshot(Request::builder().uri("/version").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let version_response: VersionResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(version_response.version, "v1.2.3");
    }
}
