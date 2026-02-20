#![cfg(feature = "rocket_support")]

use line_bot_sdk_rust::support::rocket::Signature;
use rocket::http::{Header, Status};
use rocket::local::blocking::Client;
use rocket::{post, routes};

#[post("/callback")]
fn callback(signature: Signature) -> String {
    signature.key
}

#[test]
fn extract_signature_from_valid_header() {
    let rocket = rocket::build().mount("/", routes![callback]);
    let client = Client::tracked(rocket).unwrap();

    let response = client
        .post("/callback")
        .header(Header::new("x-line-signature", "test_signature_value"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "test_signature_value");
}

#[test]
fn extract_signature_missing_header_returns_bad_request() {
    let rocket = rocket::build().mount("/", routes![callback]);
    let client = Client::tracked(rocket).unwrap();

    let response = client.post("/callback").dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}
