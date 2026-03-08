//! `/leave` command handler.
//!
//! Makes the bot leave the current group or room after sending a farewell message.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::Source,
};

/// Sends a goodbye message, then calls leave_group or leave_room depending on
/// the event source type. Does nothing in 1-on-1 chats.
pub async fn handle(
    line: &LINE,
    reply_token: String,
    source: &Option<Box<Source>>,
) -> Result<(), String> {
    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(
            "Bye everyone! See you later!".to_string(),
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    match source {
        Some(s) => match s.as_ref() {
            Source::GroupSource(g) => {
                println!("[command/leave] Leaving group {}", g.group_id);
                line.messaging_api_client
                    .leave_group(&g.group_id)
                    .await
                    .map_err(|e| format!("leave_group failed: {e}"))?;
            }
            Source::RoomSource(r) => {
                println!("[command/leave] Leaving room {}", r.room_id);
                line.messaging_api_client
                    .leave_room(&r.room_id)
                    .await
                    .map_err(|e| format!("leave_room failed: {e}"))?;
            }
            _ => {
                println!("[command/leave] Not in a group or room, ignoring");
            }
        },
        None => {
            println!("[command/leave] No source, ignoring");
        }
    }

    Ok(())
}
