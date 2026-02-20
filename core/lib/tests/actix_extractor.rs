#![cfg(feature = "actix_support")]

use actix_web::dev::Payload;
use actix_web::test::TestRequest;
use actix_web::FromRequest;
use line_bot_sdk_rust::support::actix::Signature;

#[tokio::test]
async fn extract_signature_from_valid_header() {
    let req = TestRequest::default()
        .insert_header(("x-line-signature", "test_signature_value"))
        .to_http_request();

    let sig = Signature::from_request(&req, &mut Payload::None)
        .await
        .unwrap();
    assert_eq!(sig.key, "test_signature_value");
}

#[tokio::test]
async fn extract_signature_missing_header_returns_error() {
    let req = TestRequest::default().to_http_request();

    let result = Signature::from_request(&req, &mut Payload::None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn extract_signature_debug_format() {
    let req = TestRequest::default()
        .insert_header(("x-line-signature", "abc123"))
        .to_http_request();

    let sig = Signature::from_request(&req, &mut Payload::None)
        .await
        .unwrap();
    let debug = format!("{:?}", sig);
    assert!(debug.contains("abc123"));
}
