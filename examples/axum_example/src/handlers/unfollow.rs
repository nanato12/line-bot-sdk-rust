//! Unfollow event handler.
//!
//! Triggered when a user blocks or removes the bot. No reply is possible.

use line_bot_sdk_rust::line_webhook::models::UnfollowEvent;

pub fn handle(event: UnfollowEvent) -> Result<(), String> {
    println!(
        "[unfollow] User unfollowed (event_id: {})",
        event.webhook_event_id
    );
    Ok(())
}
