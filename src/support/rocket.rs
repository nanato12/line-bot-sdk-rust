//! Support for `rocket` framework
//! # Example
//! ```
//! #![feature(proc_macro_hygiene, decl_macro)]
//!
//! #[macro_use]
//! extern crate rocket;
//!
//! extern crate line_bot_sdk_rust as line;
//! use line::support::rocket_support::{Body, Signature};
//!
//! #[post("/callback", data = "<body>")]
//! fn callback(signature: Signature, body: Body) {
//!     // more
//! }
//!
//! fn main() {
//!     rocket::ignite().mount("/", routes![callback]).launch();
//! }
//! ```
use rocket::data::{self, FromDataSimple};
use rocket::http::{ContentType, Status};
use rocket::request::{self, FromRequest};
use rocket::Outcome::{Failure, Forward, Success};
use rocket::{Data, Request};
use std::io::Read;

/// Signature in HTTP Request headers
#[derive(Debug)]
pub struct Signature {
    pub key: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Signature {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("X-Line-Signature").collect();
        match keys.len() {
            0 => Failure((Status::BadRequest, ())),
            1 => Success(Signature {
                key: keys[0].to_string(),
            }),
            _ => Failure((Status::BadRequest, ())),
        }
    }
}

/// HTTP Request body
#[derive(Debug)]
pub struct Body {
    pub string: String,
}

impl FromDataSimple for Body {
    type Error = String;
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        // ContentType must be application/json
        let body_ct = ContentType::new("application", "json");
        if req.content_type() != Some(&body_ct) {
            return Forward(data);
        }
        // Parse data to String
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }
        // Return successfully.
        Success(Body { string })
    }
}
