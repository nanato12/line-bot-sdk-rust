//! `/carousel` command handler.
//!
//! Demonstrates sending a CarouselTemplate with multiple columns.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{
            Action, CarouselColumn, CarouselTemplate, Message, MessageAction, ReplyMessageRequest,
            Template, TemplateMessage, UriAction,
        },
    },
};

/// Sends a carousel template showcasing multiple LINE API services.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let columns = vec![
        CarouselColumn {
            title: Some("Messaging API".to_string()),
            text: "Send and receive messages, manage rich menus, and more.".to_string(),
            actions: vec![
                Action::UriAction(UriAction {
                    label: Some("Documentation".to_string()),
                    uri: Some("https://developers.line.biz/en/docs/messaging-api/".to_string()),
                    ..Default::default()
                }),
                Action::MessageAction(MessageAction {
                    label: Some("Try /botinfo".to_string()),
                    text: Some("/botinfo".to_string()),
                }),
            ],
            ..CarouselColumn::new(String::new(), vec![])
        },
        CarouselColumn {
            title: Some("Flex Message".to_string()),
            text: "Build highly customizable message layouts with JSON.".to_string(),
            actions: vec![
                Action::UriAction(UriAction {
                    label: Some("Simulator".to_string()),
                    uri: Some("https://developers.line.biz/flex-simulator/".to_string()),
                    ..Default::default()
                }),
                Action::MessageAction(MessageAction {
                    label: Some("Try /flex".to_string()),
                    text: Some("/flex".to_string()),
                }),
            ],
            ..CarouselColumn::new(String::new(), vec![])
        },
        CarouselColumn {
            title: Some("LIFF".to_string()),
            text: "Build web apps that run inside the LINE client.".to_string(),
            actions: vec![
                Action::UriAction(UriAction {
                    label: Some("Documentation".to_string()),
                    uri: Some("https://developers.line.biz/en/docs/liff/".to_string()),
                    ..Default::default()
                }),
                Action::UriAction(UriAction {
                    label: Some("Playground".to_string()),
                    uri: Some("https://liff-playground.netlify.app/".to_string()),
                    ..Default::default()
                }),
            ],
            ..CarouselColumn::new(String::new(), vec![])
        },
    ];

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TemplateMessage(TemplateMessage::new(
            "LINE Platform Services".to_string(),
            Template::CarouselTemplate(CarouselTemplate::new(columns)),
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
