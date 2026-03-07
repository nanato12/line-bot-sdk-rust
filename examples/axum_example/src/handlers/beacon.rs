//! Beacon event handler.
//!
//! Triggered when a user enters, leaves, or stays in range of a LINE Beacon.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::BeaconEvent,
};

/// Replies with the detected beacon hardware ID.
pub async fn handle(line: &LINE, event: BeaconEvent) -> Result<(), String> {
    println!(
        "[beacon] hwid={} type={:?}",
        event.beacon.hwid, event.beacon.r#type
    );

    let req = ReplyMessageRequest {
        reply_token: event.reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(format!(
            "Beacon detected! (hwid: {})",
            event.beacon.hwid
        )))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
