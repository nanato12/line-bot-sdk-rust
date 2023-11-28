# LINE Messaging API SDK for Rust

## Introduction

The LINE Messaging API SDK for Rust makes it easy to develop bots using LINE Messaging API, and you can create a sample bot within minutes.

## Documentation

See the official API documentation for more information.

- English: <https://developers.line.biz/en/docs/messaging-api/overview/>
- Japanese: <https://developers.line.biz/ja/docs/messaging-api/overview/>

## Requirements

This library requires Rust nightly.

## Installation

```toml
[dependencies]
line-bot-sdk-rust = "0.1"
```

If you use `rocket support`.

```toml
[dependencies]
line-bot-sdk-rust = { version = "0.1", features = ["rocket_support"] }
```

If you use `actix_web support`.

```toml
[dependencies]
line-bot-sdk-rust = { version = "0.1", features = ["actix_support"] }
```

## Configuration

```rust
extern crate line_bot_sdk_rust as line;
use line::bot::LineBot;

fn main() {
    let bot = LineBot::new("<channel secret>", "<channel access token>");
}
```

## How to use

The LINE Messaging API uses the JSON data format.
parse_event_request() will help you to parse the HttpRequest content and return a Result<[Events](`events::Events`) , &'static str> Object.

```rust
 let result: Result<Events, &'static str> =
     bot.parse_event_request(signature, body);
```

```rust
match result {
    Ok(events) => {
        for event in events.events {
            ...
        }
    }
    Err(msg) => {}
}
```

## EchoBot examples

### with Rocket framework

```bash
cargo run --example echobot_rocket --features=rocket_support
```

source: [rocket example](./examples/echobot_rocket.rs)

### with Actix_web framework

```bash
cargo run --example echobot_actix_web --features=actix_support
```

source: [actix_web example](./examples/echobot_actix_web.rs)

## Contributing

Please make a contribution 😆

## License

```plain
Copyright 2021 nanato12

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
