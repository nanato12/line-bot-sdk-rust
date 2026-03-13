//! `/quick` command handler.
//!
//! Demonstrates sending a message with QuickReply buttons.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{
            Action, CameraAction, CameraRollAction, LocationAction, Message, MessageAction,
            PostbackAction, QuickReply, QuickReplyItem, ReplyMessageRequest, TextMessage,
        },
    },
};

/// Sends a text message with quick reply buttons demonstrating various action types.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    fn item(action: Action) -> QuickReplyItem {
        QuickReplyItem {
            action: Some(Box::new(action)),
            ..Default::default()
        }
    }

    let quick_reply = QuickReply {
        items: Some(vec![
            item(Action::MessageAction(MessageAction {
                label: Some("Hello".to_string()),
                text: Some("Hello!".to_string()),
            })),
            item(Action::PostbackAction(PostbackAction {
                label: Some("Postback".to_string()),
                data: Some("quick=postback".to_string()),
                display_text: Some("Postback tapped".to_string()),
                ..Default::default()
            })),
            item(Action::CameraAction(CameraAction {
                label: Some("Camera".to_string()),
            })),
            item(Action::CameraRollAction(CameraRollAction {
                label: Some("Album".to_string()),
            })),
            item(Action::LocationAction(LocationAction {
                label: Some("Location".to_string()),
            })),
        ]),
    };

    let mut msg = TextMessage::new("Tap a quick reply button below:".to_string());
    msg.quick_reply = Some(Box::new(quick_reply));

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TextMessage(msg)],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
