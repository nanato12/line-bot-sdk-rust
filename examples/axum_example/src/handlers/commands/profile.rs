//! `/profile` command handler.
//!
//! Fetches and displays the sender's LINE profile information.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::Source,
};

/// Looks up the sender's user ID from the event source and replies with their profile.
pub async fn handle(
    line: &LINE,
    reply_token: String,
    source: &Option<Box<Source>>,
) -> Result<(), String> {
    let user_id = match source {
        Some(s) => match s.as_ref() {
            Source::UserSource(u) => u.user_id.clone(),
            Source::GroupSource(g) => g.user_id.clone(),
            Source::RoomSource(r) => r.user_id.clone(),
            _ => None,
        },
        None => None,
    };

    let text = match user_id {
        Some(uid) => match line.messaging_api_client.get_profile(&uid).await {
            Ok(profile) => format!("{profile:#?}"),
            Err(e) => format!("Failed to get profile: {e}"),
        },
        None => "Could not determine your user ID.".to_string(),
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
