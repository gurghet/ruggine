use axum::{
    response::{Redirect, IntoResponse},
    extract::Path,
    http::StatusCode,
    body::{Body, to_bytes},
};
use tower::ServiceExt;

pub async fn url_redirect_handler(Path(code): Path<String>) -> impl IntoResponse {
    match code.as_str() {
        "B5Z" => Redirect::temporary("https://codecraft.engineering").into_response(),
        _ => (StatusCode::NOT_FOUND, "URL shortcode not found").into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        http::Request,
        Router,
        routing::get,
    };

    #[tokio::test]
    async fn test_valid_redirect() {
        let app = Router::new()
            .route("/url/:code", get(url_redirect_handler));

        let response = app
            .oneshot(Request::builder().uri("/url/B5Z").body(Body::empty()).unwrap())
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
        let app = Router::new()
            .route("/url/:code", get(url_redirect_handler));

        let response = app
            .oneshot(Request::builder().uri("/url/invalid").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(&body[..], b"URL shortcode not found");
    }
}