extern crate line_bot_sdk_rust as line;

use dotenv::dotenv;
use std::env;

use line::bot::LineBot;
use line::events::messages::MessageType;
use line::events::{EventType, Events};
use line::messages::{SendMessageType, TextMessage};
use line::support::actix_support::Signature;
use line::webhook::validate_signature;

use actix_web::{post, web, App, HttpServer, Responder};

#[post("/callback")]
async fn callback(
    signature: Signature,
    data: web::Json<Events>,
    bytes: web::Bytes,
) -> impl Responder {
    // Get channel secret and access token by environment variable
    let channel_secret: &str =
        &env::var("LINE_CHANNEL_SECRET").expect("Failed getting LINE_CHANNEL_SECRET");
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed getting LINE_CHANNEL_ACCESS_TOKEN");

    // LineBot
    let bot = LineBot::new(channel_secret, access_token);

    let body: &str = &String::from_utf8(bytes.to_vec()).unwrap();
    validate_signature(&bot.channel_secret, &signature.key, &body);

    for event in &data.events {
        // MessageEvent only
        if let EventType::MessageEvent(message_event) = &event.r#type {
            // TextMessageEvent only
            if let MessageType::TextMessage(text_message) = &message_event.message.r#type {
                // Create TextMessage
                let message = SendMessageType::TextMessage(TextMessage {
                    text: text_message.text.to_string(),
                    emojis: None,
                });
                // Reply message with reply_token
                let _res = bot.reply_message(&message_event.reply_token, vec![message]);
            }
        }
    }

    "OK"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().service(callback))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
