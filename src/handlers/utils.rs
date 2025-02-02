use axum::response::Json;
use crate::models::VersionResponse;

pub async fn version_handler() -> Json<VersionResponse> {
    let version = std::env::var("APP_VERSION").unwrap_or_else(|_| "unknown".to_string());
    Json(VersionResponse { version })
}

pub async fn healthz_handler() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        http::{Request, StatusCode},
        Router,
        body::to_bytes,
        routing::get,
    };
    use tower::ServiceExt;

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

    #[tokio::test]
    async fn test_healthz_endpoint() {
        let app = Router::new()
            .route("/healthz", get(healthz_handler));
    
        let response = app
            .oneshot(Request::builder().uri("/healthz").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();
    
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(String::from_utf8(body.to_vec()).unwrap(), "ok");
    }
}