//! `/botinfo` command handler.
//!
//! Fetches and displays the bot's own profile information.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
};

/// Retrieves the bot's info via the Messaging API and replies with a debug dump.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let text = match line.messaging_api_client.get_bot_info().await {
        Ok(info) => format!("{info:#?}"),
        Err(e) => format!("Failed to get bot info: {e}"),
    };

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
