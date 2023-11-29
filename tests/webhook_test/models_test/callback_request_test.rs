#[cfg(test)]
mod callback_request_test {
    extern crate line_bot_sdk_rust as line;

    use line::webhook::models::{
        CallbackRequest, DeliveryContext, Event, EventMode, GroupSource, MessageContent,
        MessageEvent, Source, TextMessageContent,
    };

    #[test]
    fn test_callback_request() {
        let event_body = "{\"destination\":\"destination\",\"events\":[{\"type\":\"message\",\"source\":{\"type\":\"group\",\"groupId\":\"group_id\",\"userId\":\"user_id\"},\"timestamp\":1,\"mode\":\"active\",\"webhookEventId\":\"webhook_event_id\",\"deliveryContext\":{\"isRedelivery\":false},\"replyToken\":\"reply_token\",\"message\":{\"type\":\"text\",\"id\":\"message_id\",\"text\":\"text\",\"quoteToken\":\"quote_token\"}}]}";

        let r = CallbackRequest {
            destination: "destination".to_string(),
            events: vec![Event::MessageEvent(MessageEvent {
                timestamp: 1,
                mode: EventMode::Active,
                webhook_event_id: "webhook_event_id".to_string(),
                source: Some(Box::new(Source::GroupSource(GroupSource {
                    group_id: "group_id".to_string(),
                    user_id: Some("user_id".to_string()),
                }))),
                delivery_context: Box::new(DeliveryContext {
                    is_redelivery: false,
                }),
                reply_token: Some("reply_token".to_string()),
                message: Box::new(MessageContent::TextMessageContent(TextMessageContent {
                    id: "message_id".to_string(),
                    text: "text".to_string(),
                    emojis: None,
                    mention: None,
                    quote_token: "quote_token".to_string(),
                    quoted_message_id: None,
                })),
            })],
        };

        assert_eq!(event_body, serde_json::to_string(&r).unwrap());

        let deserialized_request: CallbackRequest = serde_json::from_str(&event_body).unwrap();
        assert_eq!(r, deserialized_request);
    }
}
