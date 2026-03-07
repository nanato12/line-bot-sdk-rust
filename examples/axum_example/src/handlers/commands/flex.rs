//! `/flex` command handler.
//!
//! Sends a Flex Message sample built programmatically using the SDK's model types.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{
            flex_box::Layout, FlexBox, FlexBubble, FlexCarousel, FlexComponent, FlexContainer,
            FlexMessage, FlexText, Message, ReplyMessageRequest,
        },
    },
};

/// Builds a FlexCarousel with two identical bubbles and sends it as a reply.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    // Build a FlexContainer programmatically using the SDK's model constructors.
    let bubble = FlexBubble {
        body: Some(Box::new(FlexBox::new(
            Layout::Vertical,
            vec![FlexComponent::FlexText(FlexText {
                text: Some("hello, world".to_string()),
                ..Default::default()
            })],
        ))),
        ..FlexBubble::new()
    };

    let contents = FlexContainer::FlexCarousel(FlexCarousel::new(vec![bubble.clone(), bubble]));

    println!(
        "[flex] payload: {}",
        serde_json::to_string_pretty(&contents).unwrap_or_default()
    );

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::FlexMessage(FlexMessage::new(
            "Flex Message".to_string(),
            contents,
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
