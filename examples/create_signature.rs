extern crate line_bot_sdk_rust as line;

use line::webhook::validate_signature;

use base64::encode;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::io::Write;

// Signature confirmation
pub fn create_signature(channel_secret: &str, body: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.write_all(body.as_bytes()).unwrap();
    return encode(&mac.finalize().into_bytes());
}

fn main() {
    let channel_secret: &str = "channel_secret";
    let request_body: &str = r#"
        {
            "destination": "xxxxxxxxxx",
            "events": [
                {
                    "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
                    "type": "message",
                    "mode": "active",
                    "timestamp": 1462629479859,
                    "source": {
                        "type": "user",
                        "userId": "U4af4980629..."
                    },
                    "message": {
                        "id": "325708",
                        "type": "text",
                        "text": "@example Hello, world! (love)",
                        "emojis": [
                                {
                                    "index": 14,
                                    "length": 6,
                                    "productId": "5ac1bfd5040ab15980c9b435",
                                    "emojiId": "001"
                                }
                            ],
                        "mention": {
                            "mentionees": [
                                {
                                    "index": 0,
                                    "length": 8,
                                    "userId": "U850014438e..."
                                }
                            ]
                        }
                    }
                }
            ]
        }
    "#;
    let signature: String = create_signature(channel_secret, request_body);

    if validate_signature(channel_secret, &signature, request_body) {
        println!("Success: {}", signature);
    } else {
        println!("NG");
    }
}
