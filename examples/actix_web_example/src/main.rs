use actix_web::{
    error::ErrorBadRequest, middleware, post, web, App, Error, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{
            flex_box::Layout, flex_button::{Height, Style}, flex_text::Weight, Action, FlexBox, FlexBubble, FlexButton, FlexComponent, FlexContainer, FlexMessage, FlexText, Message, ReplyMessageRequest, TextMessage, UriAction
        },
    },
    line_webhook::models::{CallbackRequest, Event, MessageContent},
    parser::signature::validate_signature,
    support::actix::Signature,
};
use std::env;

#[post("/callback")]
async fn callback(signature: Signature, bytes: web::Bytes) -> Result<HttpResponse, Error> {
    // Get channel secret and access token by environment variable
    let channel_secret: &str =
        &env::var("LINE_CHANNEL_SECRET").expect("Failed to get LINE_CHANNEL_SECRET");
    let access_token: &str =
        &env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed to get LINE_CHANNEL_ACCESS_TOKEN");

    let line = LINE::new(access_token.to_string());

    let body: &str = &String::from_utf8(bytes.to_vec()).unwrap();

    if !validate_signature(channel_secret, &signature.key, body) {
        return Err(ErrorBadRequest("x-line-signature is invalid."));
    }

    let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_str(&body);
    match request {
        Err(err) => return Err(ErrorBadRequest(err.to_string())),
        Ok(req) => {
            // println!("req: {req:#?}");
            for e in req.events {
                if let Event::MessageEvent(message_event) = e {
                    if let MessageContent::TextMessageContent(text_message) = *message_event.message
                    {
                        let reply_message_request = ReplyMessageRequest {
                            reply_token: message_event.reply_token.unwrap(),
                            messages: vec![Message::Flex(FlexMessage::new(
                                "New Changes".to_owned(),
                                FlexContainer::Bubble(FlexBubble {
                                    r#type: "bubble".to_owned(),
                                    body: Some(Box::new(FlexBox::new(
                                        "box".to_owned(),
                                        Layout::Vertical,
                                        vec![
                                            FlexComponent::FlexText(FlexText {
                                                r#type: "text".to_owned(),
                                                text: Some("Commit Pushed".to_owned()),
                                                weight: Some(Weight::Bold),
                                                size: Some("xl".to_owned()),
                                                wrap: Some(true),
                                                ..Default::default()
                                            }),
                                            FlexComponent::FlexBox(FlexBox {
                                                r#type: "box".to_owned(),
                                                layout: Layout::Vertical,
                                                margin: Some("lg".to_owned()),
                                                spacing: Some("sm".to_owned()),
                                                contents: vec![
                                                    FlexComponent::FlexText(FlexText {
                                                        r#type: "text".to_owned(),
                                                        text: Some(
                                                            "Commit pushed to main branch"
                                                                .to_owned(),
                                                        ),
                                                        wrap: Some(true),
                                                        ..Default::default()
                                                    }),
                                                    FlexComponent::FlexBox(FlexBox {
                                                        r#type: "box".to_owned(),
                                                        layout: Layout::Baseline,
                                                        spacing: Some("sm".to_owned()),
                                                        contents: vec![
                                                            FlexComponent::FlexText(FlexText {
                                                                r#type: "text".to_owned(),
                                                                text: Some("ID".to_owned()),
                                                                color: Some("#aaaaaa".to_owned()),
                                                                size: Some("sm".to_owned()),
                                                                flex: Some(2),
                                                                ..Default::default()
                                                            }),
                                                            FlexComponent::FlexText(FlexText {
                                                                r#type: "text".to_owned(),
                                                                text: Some("1234567".to_owned()),
                                                                wrap: Some(false),
                                                                color: Some("#666666".to_owned()),
                                                                size: Some("sm".to_owned()),
                                                                flex: Some(6),
                                                                ..Default::default()
                                                            }),
                                                        ],
                                                        ..Default::default()
                                                    }),
                                                    FlexComponent::FlexBox(FlexBox{
                                                        r#type: "box".to_owned(),
                                                        layout: Layout::Baseline,
                                                        spacing: Some("sm".to_owned()),
                                                        contents: vec![
                                                            FlexComponent::FlexText(FlexText{
                                                                r#type: "text".to_owned(),
                                                                text: Some("Committer".to_owned()),
                                                                color: Some("#aaaaaa".to_owned()),
                                                                size: Some("sm".to_owned()),
                                                                flex: Some(0),
                                                                wrap: Some(true),
                                                                ..Default::default()
                                                            }),
                                                            FlexComponent::FlexText(FlexText{
                                                                r#type: "text".to_owned(),
                                                                text: Some("KayXue".to_owned()),
                                                                color:Some("#666666".to_owned()),
                                                                size: Some("sm".to_owned()),
                                                                flex: Some(5),
                                                                ..Default::default()
                                                            })
                                                        ],
                                                        ..Default::default()
                                                    }),
                                                    FlexComponent::FlexBox(FlexBox{
                                                        r#type: "box".to_owned(),
                                                        layout: Layout::Baseline,
                                                        spacing: Some("sm".to_owned()),
                                                        contents:vec![
                                                            FlexComponent::FlexText(FlexText{
                                                                r#type:"text".to_owned(),
                                                                text: Some("Message".to_owned()),
                                                                color: Some("#aaaaaa".to_owned()),
                                                                size: Some("sm".to_owned()),
                                                                flex: Some(1),
                                                                wrap: Some(true),
                                                                ..Default::default()
                                                            }),
                                                            FlexComponent::FlexText(FlexText{
                                                                r#type:"text".to_owned(),
                                                                text: Some("Commit Msg".to_owned()),
                                                                color: Some("#666666".to_owned()),
                                                                size: Some("sm".to_owned()),
                                                                flex: Some(3),
                                                                wrap: Some(false),
                                                                ..Default::default()
                                                            })
                                                        ],
                                                        ..Default::default()
                                                    }),
                                                    FlexComponent::FlexBox(FlexBox{
                                                        r#type:"box".to_owned(),
                                                        layout: Layout::Baseline,
                                                        spacing: Some("sm".to_owned()),
                                                        contents: vec![
                                                            FlexComponent::FlexText(FlexText{
                                                                r#type:"text".to_owned(),
                                                                text:Some("Time".to_owned()),
                                                                color: Some("#aaaaaa".to_owned()),
                                                                size: Some("sm".to_owned()),
                                                                flex: Some(1),
                                                                wrap: Some(true),
                                                                ..Default::default()
                                                            }),
                                                            FlexComponent::FlexText(FlexText{
                                                                r#type:"text".to_owned(),
                                                                text: Some("2023-10-01 12:00:00".to_owned()),
                                                                color: Some("#666666".to_owned()),
                                                                size: Some("sm".to_owned()),
                                                                flex: Some(3),
                                                                wrap: Some(true),
                                                                ..Default::default()
                                                            })
                                                        ],
                                                        ..Default::default()
                                                    })
                                                ],
                                                ..Default::default()
                                            }),
                                        ],
                                    ))),
                                    footer: Some(Box::new(FlexBox{
                                        r#type:"box".to_owned(),
                                        layout:Layout::Vertical,
                                        spacing:Some("sm".to_owned()),
                                        contents: vec![
                                            FlexComponent::FlexButton(FlexButton{
                                                r#type:"button".to_owned(),
                                                style: Some(Style::Link),
                                                height: Some(Height::Sm),
                                                action:Box::new(Action::URI(UriAction{
                                                    r#type:Some("uri".to_owned()),
                                                    label: Some("Check the commit".to_owned()),
                                                    uri: Some("https://www.google.com".to_owned()),
                                                    ..Default::default()
                                                })),
                                                ..Default::default()
                                            })
                                        ],
                                        ..Default::default()
                                    })),
                                    ..Default::default()
                                }),
                            ))],
                            notification_disabled: Some(false),
                        };
                        let res=serde_json::to_string_pretty(&reply_message_request.messages[0])
                            .map_err(|e| ErrorBadRequest(e.to_string()))?;
                        println!("res: {res}");
                        let result = line
                            .messaging_api_client
                            .reply_message(reply_message_request)
                            .await;
                        match result {
                            Ok(r) => println!("{:#?}", r),
                            Err(e) => println!("{:#?}", e),
                        }
                    };
                };
            }
        }
    }

    Ok(HttpResponse::Ok().body("ok"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(callback)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
