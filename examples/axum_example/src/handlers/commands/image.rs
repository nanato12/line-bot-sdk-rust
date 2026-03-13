//! `/image` command handler.
//!
//! Demonstrates sending an ImageMessage.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{ImageMessage, Message, ReplyMessageRequest},
    },
};

/// Sends a sample image message.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let url = "https://rustacean.net/assets/rustacean-flat-happy.png".to_string();

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::ImageMessage(ImageMessage::new(url.clone(), url))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
