use axum::{
    response::Redirect,
    extract::Path,
};

pub async fn url_redirect_handler(Path(code): Path<String>) -> Redirect {
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
        routing::get,
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
        let app = Router::new()
            .route("/url/:code", get(url_redirect_handler));

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
}