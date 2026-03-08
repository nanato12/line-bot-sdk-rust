//! Join event handler.
//!
//! Triggered when the bot is invited to a group or multi-person chat room.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::JoinEvent,
};

/// Sends a greeting when the bot joins a group or room.
pub async fn handle(line: &LINE, event: JoinEvent) -> Result<(), String> {
    println!("[join] Bot joined a group/room!");

    let req = ReplyMessageRequest {
        reply_token: event.reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(
            "Hello everyone! Thanks for inviting me! I'm an echo bot -- send me a text and I'll repeat it.".to_string(),
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
