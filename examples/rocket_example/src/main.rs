use dotenv::dotenv;
use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    parser::signature::validate_signature,
    support::rocket::Signature,
    line_webhook::models::{CallbackRequest, Event, MessageContent},
};

use rocket::{http::Status, launch, post, routes};
use std::env;

#[post("/callback", data = "<body>")]
async fn world(signature: Signature, body: String) -> (Status, &'static str) {
    // Get channel secret and access token by environment variable
    let channel_secret: &str =
        &env::var("LINE_CHANNEL_SECRET").expect("Failed to get LINE_CHANNEL_SECRET");
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed to get LINE_CHANNEL_ACCESS_TOKEN");

    let _line = LINE::new(access_token.to_string());

    println!("{signature:#?}");
    println!("{body:#?}");

    if !validate_signature(channel_secret.to_string(), signature.key, body.clone()) {
        return (Status::BadRequest, "x-line-signature is invalid.");
    }

    let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_str(&body);

    match request {
        Err(_) => {}
        Ok(req) => {
            println!("req: {req:#?}");
            for e in req.events {
                if let Event::MessageEvent(message_event) = e {
                    if let MessageContent::TextMessageContent(text_message) = *message_event.message
                    {
                        let _reply_message_request = ReplyMessageRequest {
                            reply_token: message_event.reply_token.unwrap(),
                            messages: vec![Message::Text(TextMessage::new(text_message.text))],
                            notification_disabled: Some(false),
                        };
                        // TODO: reply_message sample
                        // let result = line
                        //     .messaging_api_client
                        //     .reply_message(reply_message_request)
                        //     .await;
                        // match result {
                        //     Ok(r) => println!("{:#?}", r),
                        //     Err(e) => println!("{:#?}", e),
                        // }
                    };
                };
            }
        }
    }

    (Status::Ok, "OK")
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![world])
        .configure(rocket::Config::figment().merge(("port", 8080)))
}
