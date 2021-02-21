#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

extern crate line_bot_sdk_rust as line;

use dotenv::dotenv;
use std::env;

use rocket::http::Status;

use line::bot::LineBot;
use line::events::Events;
use line::support::rocket_support::{Body, Signature};

#[post("/callback", data = "<body>")]
fn callback(signature: Signature, body: Body) -> Status {
    let channel_secret: &str =
        &env::var("LINE_CHANNEL_RECRET").expect("Failed getting LINE_CHANNEL_RECRET");
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed getting LINE_CHANNEL_ACCESS_TOKEN");

    let bot = LineBot::new(channel_secret, access_token);
    let result: Result<Events, &'static str> =
        bot.parse_event_request(&signature.key, &body.string);

    if let Err(msg) = result {
        return Status::new(500, msg);
    } else if let Ok(res) = result {
        for event in res.events {
            println!("{:?}", event);
        }
        return Status::new(200, "OK");
    }
    Status::new(200, "OK")
}

fn main() {
    dotenv().ok();
    rocket::ignite().mount("/", routes![callback]).launch();
}
