//! Unsend event handler.
//!
//! Triggered when a user unsends (deletes) a message. No reply is sent.

use line_bot_sdk_rust::line_webhook::models::UnsendEvent;

pub fn handle(event: UnsendEvent) -> Result<(), String> {
    println!(
        "[unsend] Message unsent (message_id: {})",
        event.unsend.message_id
    );
    Ok(())
}
