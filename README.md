# LINE Messaging API SDK for Rust

## Introduction

The LINE Messaging API SDK for Rust makes it easy to develop bots using LINE Messaging API, and you can create a sample bot within minutes.

## Documentation

See the official API documentation for more information.

- English: <https://developers.line.biz/en/docs/messaging-api/overview/>
- Japanese: <https://developers.line.biz/ja/docs/messaging-api/overview/>

## Requirements

This library requires stable/beta Rust.

## Installation

```toml
[dependencies]
line-bot-sdk-rust = "1.0.0"
```

or

```bash
$ cargo add line-bot-sdk-rust
```

## Web framework support

Extract `x-line-signature` from the request header.

### Use `rocket` framework

```toml
[dependencies.line-bot-sdk-rust]
version = "1.0.0"
features = ["rocket_support"]
```

```rust
use line_bot_sdk_rust::support::rocket::Signature;
use rocket::{http::Status, post};

#[post("/callback", data = "<body>")]
async fn world(signature: Signature, body: String) -> (Status, &'static str) {
    ...
}
```

### Use `actix_web` framework

```toml
[dependencies.line-bot-sdk-rust]
version = "1.0.0"
features = ["actix_support"]
```

```rust
use actix_web::{post, web, Error, HttpResponse};
use line_bot_sdk_rust::support::actix::Signature;

#[post("/callback")]
async fn callback(signature: Signature, bytes: web::Bytes) -> Result<HttpResponse, Error> {
    ...
}
```

## Configuration

```rust
use line_bot_sdk_rust::client::LINE;
use std::env;

fn main() {
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed to get LINE_CHANNEL_ACCESS_TOKEN");

    let line = LINE::new(access_token.to_string());
}
```

## How to use

The LINE Messaging API uses the JSON data format.

Example. Parse body (`&str`) into Result<CallbackRequest, serde_json::Error>.

```rust
let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_str(body);
```

```rust
match request {
    Err(err) => {
        // error handling
    },
    Ok(req) => {
        for e in req.events {
            // Processing for various events
        }
    }
}
```

## EchoBot examples

### with Rocket framework

```bash
$ cd examples
$ cargo run --bin rocket
```

source: [rocket example](./examples/rocket/src/main.rs)

### with actix_web framework

```bash
$ cd examples
$ cargo run --bin actix_web
```

source: [actix_web example](./examples/actix_web/src/main.rs)

## Contributing

Please make a contribution 😆

## License

```plain
Copyright 2023 nanato12

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
