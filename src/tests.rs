use actix_web::{App, http::StatusCode, test};

use super::*;

#[actix_web::test]
async fn test_health() {
    let app = test::init_service(App::new().service(health)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(resp.headers().is_empty());

    let body = test::read_body(resp).await;
    assert!(body.is_empty());
}

#[actix_web::test]
async fn test_get_client_ip() {
    let app = test::init_service(App::new().service(get_client_ip)).await;
    let req = test::TestRequest::default()
        .insert_header(("X-Forwarded-For", "::2, 1.1.1.1"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let body = std::str::from_utf8(&body).unwrap_or_default();
    assert_eq!(body, "::2");
}
