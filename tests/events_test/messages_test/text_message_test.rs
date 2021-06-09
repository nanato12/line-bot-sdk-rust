#[cfg(test)]
mod text_message_test {
    extern crate line_bot_sdk_rust as line;

    use line::events::messages::MessageType;
    use line::events::source::SourceType;
    use line::events::{EventType, Events};

    #[test]
    fn test_parse_valid() {
        let string = r#"
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

        let events: Events = serde_json::from_str(&string).unwrap();

        // events length test
        assert_eq!(1, events.events.len());

        // event type test
        match &events.events[0].r#type {
            EventType::MessageEvent(message_event) => {
                // check events field value
                assert_eq!("xxxxxxxxxx", events.destination);

                // check message_event field value
                assert_eq!("nHuyWiB7yP5Zw52FIkcQobQuGDXCTA", message_event.reply_token);
                assert_eq!("active", message_event.mode);
                assert_eq!(1462629479859, message_event.timestamp);

                // check source type
                match &message_event.source.r#type {
                    SourceType::User(source) => {
                        assert_eq!("U4af4980629...", source.user_id);
                    }
                    _ => panic!("Expected SourceType::User"),
                }

                // check text_message_event field value
                match &message_event.message.r#type {
                    MessageType::TextMessage(text_message) => {
                        assert_eq!("325708", text_message.id);
                        assert_eq!("@example Hello, world! (love)", text_message.text);

                        // check text_message_event.emojis field value
                        match &text_message.emojis {
                            Some(emojis) => {
                                assert_eq!(14, emojis[0].index);
                                assert_eq!(6, emojis[0].length);
                                assert_eq!("5ac1bfd5040ab15980c9b435", emojis[0].product_id);
                                assert_eq!("001", emojis[0].emoji_id);
                            }
                            None => panic!("Expected text_message.emojis"),
                        };

                        // check text_message_event.mention field value
                        match &text_message.mention {
                            Some(mention) => {
                                assert_eq!(0, mention.mentionees[0].index);
                                assert_eq!(8, mention.mentionees[0].length);

                                // check text_message_event.mention.mentionees[0].user_id field value
                                match &mention.mentionees[0].user_id {
                                    Some(user_id) => {
                                        assert_eq!("U850014438e...", user_id);
                                    }
                                    _ => panic!(
                                        "Expected text_message_event.mention.mentionees[0].user_id"
                                    ),
                                }
                            }
                            None => panic!("Expected text_message_event.mention"),
                        };
                    }
                    _ => panic!("Expected MessageType::TextMessage"),
                }
            }
            _ => panic!("Expected EventType::MessageEvent"),
        }
    }
}
