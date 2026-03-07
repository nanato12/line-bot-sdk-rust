//! `/ginfo` command handler.
//!
//! Fetches and displays group summary and member count.
//! Only works when sent inside a group chat.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::Source,
};

/// Retrieves the group summary and member count, then replies with both.
/// Returns a message explaining this command is group-only if used elsewhere.
pub async fn handle(
    line: &LINE,
    reply_token: String,
    source: &Option<Box<Source>>,
) -> Result<(), String> {
    let group_id = match source {
        Some(s) => match s.as_ref() {
            Source::GroupSource(g) => Some(g.group_id.clone()),
            _ => None,
        },
        None => None,
    };

    let text = match group_id {
        Some(gid) => {
            let mut parts = Vec::new();

            match line.messaging_api_client.get_group_summary(&gid).await {
                Ok(summary) => parts.push(format!("{summary:#?}")),
                Err(e) => parts.push(format!("get_group_summary error: {e}")),
            }

            match line.messaging_api_client.get_group_member_count(&gid).await {
                Ok(count) => parts.push(format!("{count:#?}")),
                Err(e) => parts.push(format!("get_group_member_count error: {e}")),
            }

            parts.join("\n\n")
        }
        None => "This command can only be used in a group.".to_string(),
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
