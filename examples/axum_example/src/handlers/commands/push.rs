//! `/push` command handler.
//!
//! Demonstrates the Push Message API by sending a message outside of a reply.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, PushMessageRequest, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::Source,
};

/// Replies with a confirmation, then sends a separate push message to the user.
pub async fn handle(
    line: &LINE,
    reply_token: String,
    source: &Option<Box<Source>>,
) -> Result<(), String> {
    // First, reply to acknowledge the command
    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(
            "Sending you a push message...".to_string(),
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    // Then, send a push message (not a reply)
    let user_id = match source {
        Some(s) => match s.as_ref() {
            Source::UserSource(u) => u.user_id.clone(),
            Source::GroupSource(g) => g.user_id.clone(),
            Source::RoomSource(r) => r.user_id.clone(),
            _ => None,
        },
        None => None,
    };

    if let Some(uid) = user_id {
        let push_req = PushMessageRequest::new(
            uid,
            vec![Message::TextMessage(TextMessage::new(
                "This is a push message sent via the Push Message API!".to_string(),
            ))],
        );

        line.messaging_api_client
            .push_message(push_req, None)
            .await
            .map_err(|e| format!("push_message failed: {e}"))?;
    }

    Ok(())
}
