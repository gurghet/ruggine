use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::collections::HashMap;

mod handlers;
mod models;

use handlers::{root_handler, url_redirect_handler, version_handler, healthz_handler};

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
        .route("/url/:code", get(url_redirect_handler))
        .route("/version", get(version_handler))
        .route("/healthz", get(healthz_handler))
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
}