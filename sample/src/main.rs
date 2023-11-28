extern crate line_bot_sdk_rust_openapi as line;

use actix_web::{
    error::ErrorBadRequest, middleware, post, web, App, Error, HttpRequest, HttpResponse,
    HttpServer,
};
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

    let mut conf = Configuration::default();
    conf.bearer_access_token = Some(access_token.to_string());

    println!("channel_secret: {channel_secret:?}");
    println!("access_token: {access_token:?}");
    println!("signature: {signature:?}");

    let body: &str = &String::from_utf8(bytes.to_vec()).unwrap();

    if !validate_signature(channel_secret, &signature.key, body) {
        return Err(ErrorBadRequest("x-line-signature is invalid."));
    }

    let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_str(body);
    match request {
        Err(err) => return Err(ErrorBadRequest(err.to_string())),
        Ok(req) => {
            println!("req: {req:#?}");
            for e in req.events {
                println!("event: {e:#?}");
                if let Event::MessageEvent(message_event) = e {
                    if let MessageContent::TextMessageContent(text_message) = *message_event.message
                    {
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
        }
    }

    Ok(HttpResponse::Ok().body("ok"))
}

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(callback)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
