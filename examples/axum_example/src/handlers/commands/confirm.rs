//! `/confirm` command handler.
//!
//! Demonstrates sending a ConfirmTemplate.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{
            Action, ConfirmTemplate, Message, PostbackAction, ReplyMessageRequest, Template,
            TemplateMessage,
        },
    },
};

/// Sends a confirm template with Yes/No postback actions.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let confirm = ConfirmTemplate::new(
        "Do you like the LINE Bot SDK for Rust?".to_string(),
        vec![
            Action::PostbackAction(PostbackAction {
                label: Some("Yes!".to_string()),
                data: Some("answer=yes".to_string()),
                display_text: Some("Yes, I love it!".to_string()),
                ..Default::default()
            }),
            Action::PostbackAction(PostbackAction {
                label: Some("No...".to_string()),
                data: Some("answer=no".to_string()),
                display_text: Some("Not really...".to_string()),
                ..Default::default()
            }),
        ],
    );

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TemplateMessage(TemplateMessage::new(
            "Confirm".to_string(),
            Template::ConfirmTemplate(confirm),
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
