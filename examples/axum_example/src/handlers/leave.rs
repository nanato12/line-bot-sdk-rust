//! Leave event handler.
//!
//! Triggered when the bot is removed from a group or room. No reply is possible.

use line_bot_sdk_rust::line_webhook::models::LeaveEvent;

pub fn handle(event: LeaveEvent) -> Result<(), String> {
    println!(
        "[leave] Bot was removed from a group/room (event_id: {})",
        event.webhook_event_id
    );
    Ok(())
}
