extern crate line_bot_sdk_rust_openapi as line;

use actix_web::{
    error::ErrorBadRequest, middleware, post, web, App, Error, HttpRequest, HttpResponse,
    HttpServer,
};
use dotenv::dotenv;
use line::{
    parser::signature::validate_signature,
    support::actix::Signature,
    webhook::models::{
        CallbackRequest, DeliveryContext, Event, EventMode, GroupSource, MessageContent,
        MessageEvent, Source, TextMessageContent,
    },
};
use std::env;

#[post("/callback")]
async fn callback(
    signature: Signature,
    req: HttpRequest,
    bytes: web::Bytes,
) -> Result<HttpResponse, Error> {
    // Get channel secret and access token by environment variable
    let channel_secret: &str =
        &env::var("LINE_CHANNEL_SECRET").expect("Failed getting LINE_CHANNEL_SECRET");
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed getting LINE_CHANNEL_ACCESS_TOKEN");

    println!("channel_secret: {channel_secret:?}");
    println!("access_token: {access_token:?}");
    println!("signature: {signature:?}");

    let body: &str = &String::from_utf8(bytes.to_vec()).unwrap();
    println!("body: {body:?}");

    let r = CallbackRequest {
        destination: "U078cd692e67a90a66af06d18865830e3".to_string(),
        events: vec![Event::MessageEvent(MessageEvent {
            timestamp: 1,
            mode: EventMode::Active,
            webhook_event_id: "01HGACPYQE97R407J4AP7BZHB8".to_string(),
            source: Some(Box::new(
                Source::GroupSource(
                    GroupSource{
                        group_id: "C4e256f4c52df27c374275bb35f4e8e38".to_string(),
                        user_id: Some("Ue10d267e7ad66d524781ccf16ca6ddbd".to_string()),
                })
            )),
            delivery_context: Box::new(DeliveryContext {
                is_redelivery: false,
            }),
            reply_token: Some("3aa2f1b85ca8426c9c39f4f99f812329".to_string()),
            message: Box::new(MessageContent::TextMessageContent(TextMessageContent {
                id: "483728449715306614".to_string(),
                text: "ら".to_string(),
                emojis: None,
                mention: None,
                quote_token: "RBaLgQIeMAHBP-o96z6deIxus28rLKH7dvCl2xp2uFHB8-ubJNQmPwY3On5vfRs04V0yMituk5qDgcSkZMA24_em2Iu0mimIJgH0gOPkzxQ4Q-u0f06iYqUuFp5W-LkXvmvyZaE85dno-tOsKeWv-w".to_string(),
                quoted_message_id: None,
            })),
        })],
    };

    let json_string = serde_json::to_string(&r)?;
    println!("{json_string:#?}");

    let deserialized_request: CallbackRequest = serde_json::from_str(&json_string)?;
    println!("{deserialized_request:#?}");

    if !validate_signature(channel_secret, &signature.key, body) {
        return Err(ErrorBadRequest("x-line-signature is invalid."));
    }

    let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_str(body);
    match request {
        Ok(req) => println!("REQ: {req:?}"),
        Err(err) => return Err(ErrorBadRequest(err.to_string())),
    }

    println!("REQ: {req:?}");
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
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(callback)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
