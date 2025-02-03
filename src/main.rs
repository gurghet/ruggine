use axum::{
    routing::get,
    Router,
    extract::Path,
    response::IntoResponse,
    http::{Request, StatusCode},
    body::Body,
};
use std::net::SocketAddr;
use std::collections::HashMap;

mod handlers;
mod models;

use handlers::{root_handler, url_redirect_handler, version_handler, healthz_handler, static_file_handler};

#[tokio::main]
async fn main() {
    // Create static URL mappings (could be moved to a config file or database in the future)
    let mut url_map = HashMap::new();
    url_map.insert("B5Z".to_string(), "https://codecraft.engineering".to_string());

    // Build our application with routes
    let app = create_router();

    // Run our app
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/version", get(version_handler))
        .route("/healthz", get(healthz_handler))
        .route("/static/*file", get(|path: Path<String>| async move {
            static_file_handler(&path.0).await
        }))
        .route("/:code", get(url_redirect_handler))
        .fallback(|_req: Request<Body>| async move {
            (StatusCode::NOT_FOUND, "Not Found").into_response()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        http::{Request, StatusCode},
        body::Body,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_router_404() {
        let app = create_router();

        let response = app
            .oneshot(Request::builder().uri("/nonexistent").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_static_png_route() {
        let app = create_router();

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
            response.headers().get("Content-Type").unwrap(),
            "image/png"
        );
    }

    #[tokio::test]
    async fn test_static_nonexistent_file() {
        let app = create_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/static/nonexistent.png")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        
        // Check that we get the correct content type for the error
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/plain; charset=utf-8"
        );
        
        // Check that we get a proper error message
        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(&body[..], b"File not found");
    }
}