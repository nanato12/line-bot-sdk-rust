//! `/help` command handler.
//!
//! Lists all available slash commands.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
};

/// Replies with a list of all supported commands and their descriptions.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let text = "\
Available commands:
/help - Show this help message
/profile - Show your profile info
/botinfo - Show bot info
/ginfo - Show group info (group only)
/flex - Send a Flex Message sample
/leave - Leave the current group/room (group only)"
        .to_string();

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
