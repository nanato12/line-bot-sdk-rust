#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

extern crate line_bot_sdk_rust as line;

use dotenv::dotenv;
use std::env;

use rocket::http::Status;

use line::bot::LineBot;
use line::events::source::SouceType;
use line::events::{EventType, Events};
use line::messages::{SendMessageType, TextMessage};
use line::support::rocket_support::{Body, Signature};

#[post("/callback", data = "<body>")]
fn callback(signature: Signature, body: Body) -> Status {
    // Get channel secret and access token by environment variable
    let channel_secret: &str =
        &env::var("LINE_CHANNEL_RECRET").expect("Failed getting LINE_CHANNEL_RECRET");
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed getting LINE_CHANNEL_ACCESS_TOKEN");

    // LineBot
    let bot = LineBot::new(channel_secret, access_token);

    // Request body parse
    let result: Result<Events, &'static str> =
        bot.parse_event_request(&signature.key, &body.string);

    // Success parsing
    if let Ok(res) = result {
        for event in res.events {
            // MessageEvent only
            if let EventType::MessageEvent(message_event) = event.r#type {
                // Reply message with reply_token
                match message_event.source.r#type {
                    // By Group
                    SouceType::Group(source) => {
                        // Create TextMessage
                        let message = SendMessageType::TextMessage(TextMessage {
                            text: format!("Group Id: {}", source.group_id),
                            emojis: None,
                        });
                        let _res = bot.reply_message(&message_event.reply_token, vec![message]);
                    }
                    // By Room
                    SouceType::Room(source) => {
                        // Create TextMessage
                        let message = SendMessageType::TextMessage(TextMessage {
                            text: format!("Room Id: {}", source.room_id),
                            emojis: None,
                        });
                        let _res = bot.reply_message(&message_event.reply_token, vec![message]);
                    }
                    // By User
                    SouceType::User(source) => {
                        // Create TextMessage
                        let message = SendMessageType::TextMessage(TextMessage {
                            text: format!("User Id: {}", source.user_id),
                            emojis: None,
                        });
                        let _res = bot.reply_message(&message_event.reply_token, vec![message]);
                    }
                }
            }
        }
        return Status::new(200, "OK");
    }
    // Failed parsing
    else if let Err(msg) = result {
        return Status::new(500, msg);
    }
    Status::new(500, "Internal Server Error")
}

fn main() {
    dotenv().ok();
    rocket::ignite().mount("/", routes![callback]).launch();
}
