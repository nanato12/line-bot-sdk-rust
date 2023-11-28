use line_bot_sdk_rust_openapi::messaging_api::{
    apis::{
        configuration::Configuration,
        messaging_api_api::{reply_message, ReplyMessageParams},
    },
    models::{Message, ReplyMessageRequest, TextMessage},
};

#[tokio::main]
async fn main() {
    let text_message = TextMessage::new("text".to_string(), "message".to_string());
    let params = ReplyMessageParams {
        reply_message_request: ReplyMessageRequest {
            reply_token: "a".to_string(),
            messages: vec![Message::Text(text_message)],
            notification_disabled: Some(false),
        },
    };
    let result = reply_message(&Configuration::default(), params).await;
    match result {
        Ok(r) => println!("{:#?}", r),
        Err(e) => println!("{:#?}", e),
    }
    println!("Hello, world!");
}
