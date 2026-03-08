//! Member left event handler.
//!
//! Triggered when members leave a group that the bot is in. No reply is sent.

use line_bot_sdk_rust::line_webhook::models::MemberLeftEvent;

pub fn handle(event: MemberLeftEvent) -> Result<(), String> {
    let count = event.left.members.len();
    println!("[member_left] {count} member(s) left the group");
    Ok(())
}
