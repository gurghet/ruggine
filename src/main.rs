use axum::{
    routing::get,
    Router,
    response::Redirect,
    extract::Path,
};
use axum::serve;
use std::net::SocketAddr;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Create static URL mappings
    let mut url_map = HashMap::new();
    url_map.insert("B5Z".to_string(), "https://codecraft.engineering".to_string());

    // Build our application with routes
    let app = Router::new()
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
}
