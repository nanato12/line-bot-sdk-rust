//! `/richmenu` command handler.
//!
//! Lists all rich menus registered to the bot.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
};

/// Fetches the list of rich menus and replies with a summary.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let text = match line.messaging_api_client.get_rich_menu_list().await {
        Ok(list) => {
            if list.richmenus.is_empty() {
                "No rich menus found.".to_string()
            } else {
                let mut lines = vec![format!("Rich Menus ({}):", list.richmenus.len())];
                for menu in &list.richmenus {
                    lines.push(format!("- {} ({})", menu.name, menu.rich_menu_id));
                }
                lines.join("\n")
            }
        }
        Err(e) => format!("Failed to get rich menus: {e}"),
    };

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
