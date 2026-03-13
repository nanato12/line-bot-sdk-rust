//! `/members` command handler.
//!
//! Lists member IDs in the current group or room.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::Source,
};

/// Fetches group/room member IDs and replies with the list.
pub async fn handle(
    line: &LINE,
    reply_token: String,
    source: &Option<Box<Source>>,
) -> Result<(), String> {
    let text = match source.as_deref() {
        Some(Source::GroupSource(g)) => {
            match line
                .messaging_api_client
                .get_group_members_ids(&g.group_id, None)
                .await
            {
                Ok(res) => {
                    let count = res.member_ids.len();
                    let ids: Vec<_> = res.member_ids.iter().take(10).cloned().collect();
                    let mut text = format!("Group members ({count} fetched):\n");
                    for id in &ids {
                        text.push_str(&format!("- {id}\n"));
                    }
                    if count > 10 {
                        text.push_str(&format!("... and {} more", count - 10));
                    }
                    text
                }
                Err(e) => format!("Failed to get members: {e}"),
            }
        }
        Some(Source::RoomSource(r)) => {
            match line
                .messaging_api_client
                .get_room_members_ids(&r.room_id, None)
                .await
            {
                Ok(res) => {
                    let count = res.member_ids.len();
                    format!("Room members ({count} fetched)")
                }
                Err(e) => format!("Failed to get members: {e}"),
            }
        }
        _ => "This command can only be used in a group or room.".to_string(),
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
