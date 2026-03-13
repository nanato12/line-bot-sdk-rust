//! Video play complete event handler.
//!
//! Triggered when a user finishes watching a video message that has a tracking ID.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::VideoPlayCompleteEvent,
};

/// Replies with the tracking ID of the completed video.
pub async fn handle(line: &LINE, event: VideoPlayCompleteEvent) -> Result<(), String> {
    println!(
        "[video_play_complete] tracking_id={}",
        event.video_play_complete.tracking_id
    );

    let reply_token = event.reply_token;

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(format!(
            "You finished watching the video! (tracking_id: {})",
            event.video_play_complete.tracking_id
        )))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
