extern crate line_bot_sdk_rust as line;
use line::support::rocket::Signature;

use rocket::{launch, post, routes};

#[post("/callback")]
async fn world(signature: Signature) -> &'static str {
    println!("{signature:#?}");
    // println!("{body:#?}");
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![world])
        .configure(rocket::Config::figment().merge(("port", 8080)))
}
