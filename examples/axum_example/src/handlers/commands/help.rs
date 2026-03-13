//! `/help` command handler.
//!
//! Lists all available slash commands.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
};

/// Replies with a list of all supported commands and their descriptions.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let text = "\
Available commands:

[Message Types]
/flex - Flex Message (from JSON)
/image - Image message
/location - Location message
/template - Buttons template
/confirm - Confirm template
/carousel - Carousel template
/quick - Quick reply buttons

[API Features]
/profile - Your profile info
/botinfo - Bot info
/ginfo - Group info (group only)
/members - Group member list (group only)
/push - Push message demo
/loading - Loading animation
/quota - Message quota info
/richmenu - Rich menu list
/webhook - Webhook endpoint info

[Actions]
/leave - Leave group/room (group only)
/help - Show this help"
        .to_string();

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
