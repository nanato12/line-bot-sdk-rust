#[cfg(test)]
mod text_message_test {
    extern crate line_bot_sdk_rust as line;

    use line::messages::{Emoji, SendMessage, SendMessageType, TextMessage};
    use serde_json::{json, Value};

    #[test]
    fn test_text_message_valid() {
        let expected_str: &str = r#"
            {
                "type": "text",
                "text": "Hello, world"
            }
        "#;
        let expected_json: Value = serde_json::from_str(expected_str).unwrap();

        let message: SendMessage = SendMessage {
            r#type: SendMessageType::TextMessage(TextMessage {
                text: String::from("Hello, world"),
                emojis: None,
            }),
        };
        assert_eq!(json!(message), expected_json);
    }

    #[test]
    fn test_text_message_invalid() {
        let expected_str: &str = r#"
            {
                "type": "text",
                "text": "Hello, world"
            }
        "#;
        let expected_json: Value = serde_json::from_str(expected_str).unwrap();

        let message: SendMessage = SendMessage {
            r#type: SendMessageType::TextMessage(TextMessage {
                text: String::from("xxxxxxx"),
                emojis: None,
            }),
        };
        assert_ne!(json!(message), expected_json);
    }
    #[test]
    fn test_text_message_with_emoji_valid() {
        let expected_str: &str = r#"
            {
                "type": "text",
                "text": "$ LINE emoji $",
                "emojis": [
                    {
                        "index": 0,
                        "productId": "5ac1bfd5040ab15980c9b435",
                        "emojiId": "001"
                    },
                    {
                        "index": 13,
                        "productId": "5ac1bfd5040ab15980c9b435",
                        "emojiId": "002"
                    }
                ]
            }
        "#;
        let expected_json: Value = serde_json::from_str(expected_str).unwrap();

        let message: SendMessage = SendMessage {
            r#type: SendMessageType::TextMessage(TextMessage {
                text: String::from("$ LINE emoji $"),
                emojis: Some(vec![
                    Emoji {
                        index: 0,
                        product_id: String::from("5ac1bfd5040ab15980c9b435"),
                        emoji_id: String::from("001"),
                    },
                    Emoji {
                        index: 13,
                        product_id: String::from("5ac1bfd5040ab15980c9b435"),
                        emoji_id: String::from("002"),
                    },
                ]),
            }),
        };
        assert_eq!(json!(message), expected_json);
    }

    #[test]
    fn test_text_message_with_emoji_invalid() {
        let expected_str: &str = r#"
            {
                "type": "text",
                "text": "$ LINE emoji $",
                "emojis": [
                    {
                        "index": 0,
                        "productId": "5ac1bfd5040ab15980c9b435",
                        "emojiId": "001"
                    },
                    {
                        "index": 13,
                        "productId": "5ac1bfd5040ab15980c9b435",
                        "emojiId": "002"
                    }
                ]
            }
        "#;
        let expected_json: Value = serde_json::from_str(expected_str).unwrap();
        let message: SendMessage = SendMessage {
            r#type: SendMessageType::TextMessage(TextMessage {
                text: String::from("xxxxxxx"),
                emojis: Some(vec![
                    Emoji {
                        index: 1,
                        product_id: String::from("xxxxxxxxxxxxxxx"),
                        emoji_id: String::from("001"),
                    },
                    Emoji {
                        index: 12,
                        product_id: String::from("xxxxxxxxxxxxxxx"),
                        emoji_id: String::from("002"),
                    },
                ]),
            }),
        };
        assert_ne!(json!(message), expected_json);
    }
}
