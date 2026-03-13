//! `/template` command handler.
//!
//! Demonstrates sending a ButtonsTemplate with various action types.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{
            Action, ButtonsTemplate, Message, MessageAction, PostbackAction, ReplyMessageRequest,
            Template, TemplateMessage, UriAction,
        },
    },
};

/// Sends a buttons template with postback, message, and URI actions.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let buttons = ButtonsTemplate {
        title: Some("SDK Actions Demo".to_string()),
        text: "Choose an action type:".to_string(),
        actions: vec![
            Action::PostbackAction(PostbackAction {
                label: Some("Postback".to_string()),
                data: Some("action=postback&item=1".to_string()),
                display_text: Some("Postback tapped!".to_string()),
                ..Default::default()
            }),
            Action::MessageAction(MessageAction {
                label: Some("Message".to_string()),
                text: Some("Hello from MessageAction!".to_string()),
            }),
            Action::UriAction(UriAction {
                label: Some("Open GitHub".to_string()),
                uri: Some("https://github.com/nanato12/line-bot-sdk-rust".to_string()),
                ..Default::default()
            }),
        ],
        ..ButtonsTemplate::new(String::new(), vec![])
    };

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TemplateMessage(TemplateMessage::new(
            "Actions Demo".to_string(),
            Template::ButtonsTemplate(buttons),
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
