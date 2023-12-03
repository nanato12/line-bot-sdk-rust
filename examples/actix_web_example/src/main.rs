use actix_web::{
    error::ErrorBadRequest, middleware, post, web, App, Error, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use line_bot_sdk_rust::{
    client::LINE,
    messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    parser::signature::validate_signature,
    support::actix::Signature,
    webhook::models::{CallbackRequest, Event, MessageContent},
};
use std::env;

#[post("/callback")]
async fn callback(signature: Signature, bytes: web::Bytes) -> Result<HttpResponse, Error> {
    // Get channel secret and access token by environment variable
    let channel_secret: &str =
        &env::var("LINE_CHANNEL_SECRET").expect("Failed getting LINE_CHANNEL_SECRET");
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed getting LINE_CHANNEL_ACCESS_TOKEN");

    let line = LINE::new(access_token.to_string());

    let body: &str = &String::from_utf8(bytes.to_vec()).unwrap();

    if !validate_signature(channel_secret.to_string(), signature.key, body.to_string()) {
        return Err(ErrorBadRequest("x-line-signature is invalid."));
    }

    let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_str(&body);
    match request {
        Err(err) => return Err(ErrorBadRequest(err.to_string())),
        Ok(req) => {
            println!("req: {req:#?}");
            for e in req.events {
                if let Event::MessageEvent(message_event) = e {
                    if let MessageContent::TextMessageContent(text_message) = *message_event.message
                    {
                        let reply_message_request = ReplyMessageRequest {
                            reply_token: message_event.reply_token.unwrap(),
                            messages: vec![Message::Text(TextMessage::new(text_message.text))],
                            notification_disabled: Some(false),
                        };
                        let result = line
                            .messaging_api_client
                            .reply_message(reply_message_request)
                            .await;
                        match result {
                            Ok(r) => println!("{:#?}", r),
                            Err(e) => println!("{:#?}", e),
                        }
                    };
                };
            }
        }
    }

    Ok(HttpResponse::Ok().body("ok"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(callback)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
