//! `/location` command handler.
//!
//! Demonstrates sending a LocationMessage.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{LocationMessage, Message, ReplyMessageRequest},
    },
};

/// Sends the LINE Corporation headquarters as a sample location message.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::LocationMessage(LocationMessage::new(
            "LINE Corporation".to_string(),
            "4-1-6 Shinjuku, Shinjuku-ku, Tokyo".to_string(),
            35.6895,
            139.7004,
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
