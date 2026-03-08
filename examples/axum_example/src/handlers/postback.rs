//! Postback event handler.
//!
//! Triggered when a user taps a postback action button (e.g., in a rich menu
//! or template message).

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::PostbackEvent,
};

/// Replies with the postback data and any associated parameters.
pub async fn handle(line: &LINE, event: PostbackEvent) -> Result<(), String> {
    let reply_token = event.reply_token.ok_or("No reply token")?;

    println!("[postback] data={}", event.postback.data);

    let mut text = format!("Postback received!\ndata: {}", event.postback.data);
    if let Some(params) = &event.postback.params {
        for (k, v) in params {
            text.push_str(&format!("\n{k}: {v}"));
        }
    }

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(text))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
