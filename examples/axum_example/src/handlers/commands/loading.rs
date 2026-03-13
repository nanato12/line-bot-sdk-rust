//! `/loading` command handler.
//!
//! Demonstrates the Show Loading Animation API.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, ShowLoadingAnimationRequest, TextMessage},
    },
    line_webhook::models::Source,
};

/// Shows a loading animation to the user, then replies after a brief delay.
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

    if let Some(uid) = user_id {
        let mut loading_req = ShowLoadingAnimationRequest::new(uid);
        loading_req.loading_seconds = Some(5);

        if let Err(e) = line
            .messaging_api_client
            .show_loading_animation(loading_req)
            .await
        {
            eprintln!("[loading] show_loading_animation failed: {e}");
        }
    }

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(
            "Loading animation was displayed for 5 seconds!".to_string(),
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
