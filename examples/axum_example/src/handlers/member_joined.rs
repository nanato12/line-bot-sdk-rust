//! Member joined event handler.
//!
//! Triggered when new members join a group that the bot is in.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::MemberJoinedEvent,
};

/// Sends a welcome message for new group members.
pub async fn handle(line: &LINE, event: MemberJoinedEvent) -> Result<(), String> {
    let count = event.joined.members.len();
    println!("[member_joined] {count} member(s) joined the group");

    let text = if count == 1 {
        "Welcome to the group! Nice to meet you!".to_string()
    } else {
        format!("Welcome to the group, {count} new members! Nice to meet you all!")
    };

    let req = ReplyMessageRequest {
        reply_token: event.reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(text))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
