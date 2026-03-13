//! `/quota` command handler.
//!
//! Displays the bot's message quota and current consumption.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
};

/// Fetches message quota and consumption, then replies with the information.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let mut parts = Vec::new();

    match line.messaging_api_client.get_message_quota().await {
        Ok(quota) => parts.push(format!("[Quota]\n{quota:#?}")),
        Err(e) => parts.push(format!("[Quota] Error: {e}")),
    }

    match line
        .messaging_api_client
        .get_message_quota_consumption()
        .await
    {
        Ok(consumption) => parts.push(format!("[Consumption]\n{consumption:#?}")),
        Err(e) => parts.push(format!("[Consumption] Error: {e}")),
    }

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(parts.join("\n\n")))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
