//! Top-level event dispatcher.
//!
//! Routes each webhook event type to its corresponding handler module.

use line_bot_sdk_rust::{client::LINE, line_webhook::models::Event};

use crate::handlers;

/// Dispatches a webhook event to the appropriate handler.
///
/// Matches on the event type and delegates to the corresponding handler module.
/// Any errors returned by handlers are logged to stderr.
pub async fn handle_event(line: &LINE, event: Event) {
    let result = match event {
        Event::MessageEvent(e) => handlers::message::handle(line, e).await,
        Event::FollowEvent(e) => handlers::follow::handle(line, e).await,
        Event::UnfollowEvent(e) => handlers::unfollow::handle(e),
        Event::JoinEvent(e) => handlers::join::handle(line, e).await,
        Event::LeaveEvent(e) => handlers::leave::handle(e),
        Event::MemberJoinedEvent(e) => handlers::member_joined::handle(line, e).await,
        Event::MemberLeftEvent(e) => handlers::member_left::handle(e),
        Event::PostbackEvent(e) => handlers::postback::handle(line, e).await,
        Event::UnsendEvent(e) => handlers::unsend::handle(e),
        Event::BeaconEvent(e) => handlers::beacon::handle(line, e).await,
        Event::VideoPlayCompleteEvent(e) => handlers::video_play_complete::handle(line, e).await,
        Event::AccountLinkEvent(e) => handlers::account_link::handle(line, e).await,
        _ => {
            println!("[unhandled event]");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("[error] {e}");
    }
}
