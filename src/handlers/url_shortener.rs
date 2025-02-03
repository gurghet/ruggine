use axum::{
    response::{Redirect, IntoResponse},
    extract::Path,
    http::StatusCode,
};

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
        body::{Body, to_bytes},
        http::Request,
        Router,
        routing::get,
    };
    use tower::ServiceExt;

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

    #[tokio::test]
    async fn test_non_url_path_returns_404() {
        let app = Router::new()
            .route("/:code", get(url_redirect_handler));

        let response = app
            .oneshot(Request::builder().uri("/some-random-path").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}