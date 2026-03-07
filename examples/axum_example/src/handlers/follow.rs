//! Follow event handler.
//!
//! Triggered when a user adds the bot as a friend.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::FollowEvent,
};

/// Sends a greeting message when a user adds the bot as a friend.
pub async fn handle(line: &LINE, event: FollowEvent) -> Result<(), String> {
    println!("[follow] New friend added!");

    let req = ReplyMessageRequest {
        reply_token: event.reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(
            "Thanks for adding me as a friend! I'm an echo bot built with line-bot-sdk-rust. Send me a message and I'll echo it back!".to_string(),
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
