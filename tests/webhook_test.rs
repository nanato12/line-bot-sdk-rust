#[cfg(test)]
mod webhook_test {
    extern crate line_bot_sdk_rust as line;

    use line::webhook;

    #[test]
    fn test_validate_signature_valid() {
        let channel_secret: &str = "channel_secret";
        let signature: &str = "+uCVw4g3BuIOKnlseFJAYyFoVBFaMjxP8KXBbnIRRI0=";
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
        assert_eq!(
            true,
            webhook::validate_signature(channel_secret, signature, request_body)
        );
    }

    #[test]
    fn test_validate_signature_invalid() {
        let channel_secret: &str = "hogehoge";
        let signature: &str = "+uCVw4g3BuIOKnlseFJAYyFoVBFaMjxP8KXBbnIRRI0=";
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
        assert_eq!(
            false,
            webhook::validate_signature(channel_secret, signature, request_body)
        );
    }

    #[test]
    fn test_validate_signature_invalid_2() {
        let channel_secret: &str = "channel_secret";
        let signature: &str = "xxxxxxxxxxxxx";
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
        assert_eq!(
            false,
            webhook::validate_signature(channel_secret, signature, request_body)
        );
    }

    #[test]
    fn test_validate_signature_invalid_3() {
        let channel_secret: &str = "channel_secret";
        let signature: &str = "+uCVw4g3BuIOKnlseFJAYyFoVBFaMjxP8KXBbnIRRI0=";
        let request_body: &str = r#"
        "#;
        assert_eq!(
            false,
            webhook::validate_signature(channel_secret, signature, request_body)
        );
    }
}
