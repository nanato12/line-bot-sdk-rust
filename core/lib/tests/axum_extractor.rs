#![cfg(feature = "axum_support")]

use axum::extract::FromRequestParts;
use axum::http::Request;
use line_bot_sdk_rust::support::axum::Signature;

#[tokio::test]
async fn extract_signature_from_valid_header() {
    let (mut parts, _body) = Request::builder()
        .header("x-line-signature", "test_signature_value")
        .body(())
        .unwrap()
        .into_parts();
    let sig = Signature::from_request_parts(&mut parts, &())
        .await
        .unwrap();
    assert_eq!(sig.key, "test_signature_value");
}

#[tokio::test]
async fn extract_signature_missing_header_returns_error() {
    let (mut parts, _body) = Request::builder().body(()).unwrap().into_parts();
    let result = Signature::from_request_parts(&mut parts, &()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn extract_signature_debug_format() {
    let (mut parts, _body) = Request::builder()
        .header("x-line-signature", "abc123")
        .body(())
        .unwrap()
        .into_parts();
    let sig = Signature::from_request_parts(&mut parts, &())
        .await
        .unwrap();
    let debug = format!("{:?}", sig);
    assert!(debug.contains("abc123"));
}
