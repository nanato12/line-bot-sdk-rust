extern crate line_bot_sdk_rust as line;

use dotenv::dotenv;
use line::{
    messaging_api::{
        apis::{
            configuration::Configuration,
            messaging_api_api::{reply_message, ReplyMessageParams},
        },
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    parser::signature::validate_signature,
    support::rocket::Signature,
    webhook::models::{CallbackRequest, Event, MessageContent},
};
use rocket::{http::Status, launch, post, routes, serde::json::Json};
use std::env;

#[post("/callback", data = "<body>")]
async fn world(signature: Signature, body: Json<CallbackRequest>) -> (Status, &'static str) {
    // Get channel secret and access token by environment variable
    let channel_secret: &str =
        &env::var("LINE_CHANNEL_SECRET").expect("Failed getting LINE_CHANNEL_SECRET");
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed getting LINE_CHANNEL_ACCESS_TOKEN");

    let mut conf = Configuration::default();
    conf.bearer_access_token = Some(access_token.to_string());

    println!("{signature:#?}");
    println!("{body:#?}");

    let req = body.into_inner();

    println!("{}", serde_json::to_string(&req).unwrap());

    if !validate_signature(
        channel_secret,
        &signature.key,
        &serde_json::to_string(&req).unwrap(),
    ) {
        // return (Status::BadRequest, "x-line-signature is invalid.");
    }

    for e in req.events {
        if let Event::MessageEvent(message_event) = e {
            if let MessageContent::TextMessageContent(text_message) = *message_event.message {
                let params = ReplyMessageParams {
                    reply_message_request: ReplyMessageRequest {
                        reply_token: message_event.reply_token.unwrap(),
                        messages: vec![Message::Text(TextMessage::new(text_message.text))],
                        notification_disabled: Some(false),
                    },
                };
                let result = reply_message(&conf, params).await;
                match result {
                    Ok(r) => println!("{:#?}", r),
                    Err(e) => println!("{:#?}", e),
                }
            };
        };
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
