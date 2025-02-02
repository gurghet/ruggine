use axum::response::Html;

// Constants
const STATIC_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>Hi</title>
</head>
<body>
</body>
</html>"#;

pub async fn root_handler() -> Html<&'static str> {
    Html(STATIC_HTML)
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
    use axum::http::header::CONTENT_TYPE;

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
}