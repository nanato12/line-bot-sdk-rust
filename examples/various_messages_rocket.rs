#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate line_bot_sdk_rust as line;

use dotenv::dotenv;
use std::env;

use rocket::http::Status;

use line::bot::LineBot;
use line::events::{EventType, Events};
use line::messages::Emoji;
use line::messages::{SendMessageType, StickerMessage, TextMessage};
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
                // Create Vec of messages to be sent
                let mut messages: Vec<SendMessageType> = Vec::new();
                // Add TextMessage
                messages.push(SendMessageType::TextMessage(TextMessage {
                    text: String::from("text message"),
                    emojis: None,
                }));
                // Add TextMessage with Emoji
                messages.push(SendMessageType::TextMessage(TextMessage {
                    text: String::from("$ EMOJI!"),
                    emojis: Some(vec![Emoji {
                        index: 0,
                        product_id: String::from("5ac1bfd5040ab15980c9b435"),
                        emoji_id: String::from("001"),
                    }]),
                }));
                // Add StickerMessage
                messages.push(SendMessageType::StickerMessage(StickerMessage {
                    package_id: String::from("1"),
                    sticker_id: String::from("1"),
                }));
                println!("{:?}", messages);
                // Reply message with reply_token
                let _res = bot.reply_message(&message_event.reply_token, messages);
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
