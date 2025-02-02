use axum::{
    routing::get,
    Router,
    response::Redirect,
    extract::Path,
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

async fn root_handler() -> &'static str {
    STATIC_HTML
}

#[tokio::main]
async fn main() {
    // Create static URL mappings
    let mut url_map = HashMap::new();
    url_map.insert("B5Z".to_string(), "https://codecraft.engineering".to_string());

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/url/:code", get(url_redirect_handler));

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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        http::{Request, StatusCode},
        Router,
        body::to_bytes,
    };
    use tower::ServiceExt;

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
        
        let body = to_bytes(response.into_body(), 1024 * 32).await.unwrap();
        assert_eq!(
            String::from_utf8(body.to_vec()).unwrap(),
            STATIC_HTML
        );
    }
}
