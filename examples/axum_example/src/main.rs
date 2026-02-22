use axum::{routing::post, Router};
use dotenvy::dotenv;
use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::{CallbackRequest, Event, MessageContent},
    parser::signature::validate_signature,
    support::axum::Signature,
};
use std::env;

async fn callback(
    signature: Signature,
    body: String,
) -> Result<&'static str, (axum::http::StatusCode, String)> {
    let channel_secret =
        env::var("LINE_CHANNEL_SECRET").expect("Failed to get LINE_CHANNEL_SECRET");
    let access_token =
        env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed to get LINE_CHANNEL_ACCESS_TOKEN");

    let line = LINE::new(access_token);

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

    println!("req: {request:#?}");

    for e in request.events {
        if let Event::MessageEvent(message_event) = e {
            if let MessageContent::TextMessageContent(text_message) = *message_event.message {
                let reply_message_request = ReplyMessageRequest {
                    reply_token: message_event.reply_token.unwrap(),
                    messages: vec![Message::Text(TextMessage::new(text_message.text))],
                    notification_disabled: Some(false),
                };
                let result = line
                    .messaging_api_client
                    .reply_message(reply_message_request)
                    .await;
                match result {
                    Ok(r) => println!("{r:#?}"),
                    Err(e) => println!("{e:#?}"),
                }
            }
        }
    }

    Ok("ok")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/callback", post(callback));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
