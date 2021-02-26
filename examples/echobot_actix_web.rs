extern crate line_bot_sdk_rust as line;

use dotenv::dotenv;
use std::env;

use line::bot::LineBot;
use line::events::messages::MessageType as EventMessageType;
use line::events::{EventType, Events};
use line::messages::{SendMessageType, TextMessage};

use actix_web::web::Bytes;
use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[post("/callback")]
async fn callback(req: HttpRequest, bytes: Bytes) -> impl Responder {
    let body: &str = &String::from_utf8(bytes.to_vec()).unwrap();
    let signature: &str = req
        .headers()
        .get("x-line-signature")
        .unwrap()
        .to_str()
        .unwrap();

    // Get channel secret and access token by environment variable
    let channel_secret: &str =
        &env::var("LINE_CHANNEL_RECRET").expect("Failed getting LINE_CHANNEL_RECRET");
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed getting LINE_CHANNEL_ACCESS_TOKEN");

    // LineBot
    let bot = LineBot::new(channel_secret, access_token);

    // Request body parse
    let result: Result<Events, &'static str> = bot.parse_event_request(signature, body);

    // Success parsing
    if let Ok(res) = result {
        for event in res.events {
            // MessageEvent only
            if let EventType::MessageEvent(message_event) = event.r#type {
                // TextMessageEvent only
                if let EventMessageType::TextMessage(text_message) = message_event.message.r#type {
                    // Create TextMessage
                    let message = SendMessageType::TextMessage(TextMessage {
                        text: text_message.text,
                        emojis: None,
                    });
                    // Reply message with reply_token
                    let _res = bot.reply_message(&message_event.reply_token, vec![message]);
                }
            }
        }
        return HttpResponse::Ok().body("OK");
    }
    // Failed parsing
    else if let Err(msg) = result {
        return HttpResponse::BadRequest().body(msg);
    }
    HttpResponse::BadRequest().body("Internal Server Error")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().service(callback))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
