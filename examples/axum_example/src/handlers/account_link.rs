//! Account link event handler.
//!
//! Triggered when a user completes or fails the account linking flow.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::AccountLinkEvent,
};

/// Replies with the account linking result (success/failure) and nonce.
pub async fn handle(line: &LINE, event: AccountLinkEvent) -> Result<(), String> {
    let link = &event.link;
    println!(
        "[account_link] result={:?} nonce={}",
        link.result, link.nonce
    );

    let reply_token = event.reply_token.ok_or("No reply token")?;

    let text = format!(
        "Account linking {}\nnonce: {}",
        match link.result {
            line_bot_sdk_rust::line_webhook::models::link_content::Result::Ok => "succeeded!",
            _ => "failed.",
        },
        link.nonce
    );

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
