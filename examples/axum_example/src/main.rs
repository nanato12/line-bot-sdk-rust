//! Axum example for line-bot-sdk-rust.
//!
//! This example demonstrates how to build a LINE bot using the Axum web framework.
//! It handles webhook callbacks, validates signatures, and dispatches events
//! to the appropriate handlers.

mod handler;
mod handlers;

use axum::{routing::post, Router};
use dotenvy::dotenv;
use line_bot_sdk_rust::{
    client::LINE, line_webhook::models::CallbackRequest, parser::signature::validate_signature,
    support::axum::Signature,
};
use std::env;

/// Webhook callback endpoint.
///
/// Receives LINE webhook events, validates the signature, parses the request,
/// and dispatches each event to the appropriate handler.
async fn callback(
    signature: Signature,
    body: String,
) -> Result<&'static str, (axum::http::StatusCode, String)> {
    let channel_secret =
        env::var("LINE_CHANNEL_SECRET").expect("Failed to get LINE_CHANNEL_SECRET");
    let access_token =
        env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed to get LINE_CHANNEL_ACCESS_TOKEN");

    let line = LINE::new(access_token);

    // Verify the request signature using the channel secret
    if !validate_signature(&channel_secret, &signature.key, &body) {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "x-line-signature is invalid.".to_string(),
        ));
    }

    let request: CallbackRequest = serde_json::from_str(&body).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("JSON parse error: {e}"),
        )
    })?;

    // Dispatch each event to the appropriate handler
    for event in request.events {
        handler::handle_event(&line, event).await;
    }

    Ok("ok")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/callback", post(callback));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
