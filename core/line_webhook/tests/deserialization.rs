use line_webhook::models::{
    CallbackRequest, DeliveryContext, Event, EventMode, MessageContent, Source, UserSource,
};

// ---------------------------------------------------------------------------
// Source enum
// ---------------------------------------------------------------------------

#[test]
fn deserialize_user_source() {
    let json = r#"{"type":"user","userId":"U1234567890abcdef1234567890abcdef"}"#;
    let source: Source = serde_json::from_str(json).unwrap();
    match &source {
        Source::UserSource(s) => {
            // When deserialized via tagged enum, serde strips "type" so it defaults to "".
            assert_eq!(s.r#type, "");
            assert_eq!(
                s.user_id.as_deref(),
                Some("U1234567890abcdef1234567890abcdef")
            );
        }
        _ => panic!("expected UserSource"),
    }
}

#[test]
fn deserialize_group_source() {
    let json = r#"{"type":"group","groupId":"C1234567890abcdef1234567890abcdef","userId":"U1234567890abcdef1234567890abcdef"}"#;
    let source: Source = serde_json::from_str(json).unwrap();
    match &source {
        Source::GroupSource(s) => {
            assert_eq!(s.group_id, "C1234567890abcdef1234567890abcdef");
            assert_eq!(
                s.user_id.as_deref(),
                Some("U1234567890abcdef1234567890abcdef")
            );
        }
        _ => panic!("expected GroupSource"),
    }
}

#[test]
fn deserialize_group_source_without_user_id() {
    let json = r#"{"type":"group","groupId":"C1234567890abcdef1234567890abcdef"}"#;
    let source: Source = serde_json::from_str(json).unwrap();
    match &source {
        Source::GroupSource(s) => {
            assert_eq!(s.group_id, "C1234567890abcdef1234567890abcdef");
            assert!(s.user_id.is_none());
        }
        _ => panic!("expected GroupSource"),
    }
}

#[test]
fn deserialize_room_source() {
    let json = r#"{"type":"room","roomId":"R1234567890abcdef1234567890abcdef"}"#;
    let source: Source = serde_json::from_str(json).unwrap();
    match &source {
        Source::RoomSource(s) => {
            assert_eq!(s.room_id, "R1234567890abcdef1234567890abcdef");
            assert!(s.user_id.is_none());
        }
        _ => panic!("expected RoomSource"),
    }
}

// ---------------------------------------------------------------------------
// Direct serialization of discriminator children
// ---------------------------------------------------------------------------

#[test]
fn user_source_direct_serialize_includes_type() {
    let source = UserSource::new();
    let json = serde_json::to_string(&source).unwrap();
    assert!(json.contains(r#""type":"user""#));
}

#[test]
fn user_source_standalone_deserialize_preserves_type() {
    let json = r#"{"type":"user","userId":"U1234567890abcdef1234567890abcdef"}"#;
    let source: UserSource = serde_json::from_str(json).unwrap();
    assert_eq!(source.r#type, "user");
    let serialized = serde_json::to_string(&source).unwrap();
    assert!(serialized.contains(r#""type":"user""#));
}

// ---------------------------------------------------------------------------
// MessageContent enum
// ---------------------------------------------------------------------------

#[test]
fn deserialize_text_message_content() {
    let json = r#"{
        "type": "text",
        "id": "12345678901234",
        "text": "Hello, world!",
        "quoteToken": "q1234567890abcdef"
    }"#;
    let msg: MessageContent = serde_json::from_str(json).unwrap();
    match &msg {
        MessageContent::TextMessageContent(t) => {
            assert_eq!(t.id, "12345678901234");
            assert_eq!(t.text, "Hello, world!");
            assert_eq!(t.quote_token, "q1234567890abcdef");
            assert!(t.emojis.is_none());
            assert!(t.mention.is_none());
        }
        _ => panic!("expected TextMessageContent"),
    }
    // roundtrip
    let serialized = serde_json::to_string(&msg).unwrap();
    let roundtrip: MessageContent = serde_json::from_str(&serialized).unwrap();
    assert_eq!(msg, roundtrip);
}

#[test]
fn deserialize_sticker_message_content() {
    let json = r#"{
        "type": "sticker",
        "id": "12345678901234",
        "packageId": "1",
        "stickerId": "1",
        "stickerResourceType": "STATIC",
        "quoteToken": "q1234567890abcdef"
    }"#;
    let msg: MessageContent = serde_json::from_str(json).unwrap();
    match &msg {
        MessageContent::StickerMessageContent(s) => {
            assert_eq!(s.id, "12345678901234");
            assert_eq!(s.package_id, "1");
            assert_eq!(s.sticker_id, "1");
            assert_eq!(
                s.sticker_resource_type,
                line_webhook::models::sticker_message_content::StickerResourceType::Static
            );
        }
        _ => panic!("expected StickerMessageContent"),
    }
}

#[test]
fn deserialize_image_message_content() {
    let json = r#"{
        "type": "image",
        "id": "12345678901234",
        "contentProvider": {"type": "line"},
        "quoteToken": "q1234567890abcdef"
    }"#;
    let msg: MessageContent = serde_json::from_str(json).unwrap();
    match &msg {
        MessageContent::ImageMessageContent(i) => {
            assert_eq!(i.id, "12345678901234");
            assert_eq!(
                i.content_provider.r#type,
                line_webhook::models::content_provider::Type::Line
            );
        }
        _ => panic!("expected ImageMessageContent"),
    }
}

// ---------------------------------------------------------------------------
// Event enum
// ---------------------------------------------------------------------------

#[test]
fn deserialize_text_message_event() {
    let json = r#"{
        "type": "message",
        "source": {
            "type": "user",
            "userId": "U1234567890abcdef1234567890abcdef"
        },
        "timestamp": 1704067200000,
        "mode": "active",
        "webhookEventId": "01HKTEST1234567890ABCDEFGH",
        "deliveryContext": {"isRedelivery": false},
        "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
        "message": {
            "type": "text",
            "id": "12345678901234",
            "text": "Hello!",
            "quoteToken": "q1234567890abcdef"
        }
    }"#;
    let event: Event = serde_json::from_str(json).unwrap();
    match &event {
        Event::MessageEvent(e) => {
            assert_eq!(e.timestamp, 1704067200000);
            assert_eq!(e.mode, EventMode::Active);
            assert_eq!(e.webhook_event_id, "01HKTEST1234567890ABCDEFGH");
            assert!(!e.delivery_context.is_redelivery);
            assert_eq!(
                e.reply_token.as_deref(),
                Some("nHuyWiB7yP5Zw52FIkcQobQuGDXCTA")
            );
            match e.source.as_deref() {
                Some(Source::UserSource(u)) => {
                    assert_eq!(
                        u.user_id.as_deref(),
                        Some("U1234567890abcdef1234567890abcdef")
                    );
                }
                _ => panic!("expected UserSource"),
            }
            match e.message.as_ref() {
                MessageContent::TextMessageContent(t) => {
                    assert_eq!(t.text, "Hello!");
                }
                _ => panic!("expected TextMessageContent"),
            }
        }
        _ => panic!("expected MessageEvent"),
    }
}

#[test]
fn deserialize_follow_event() {
    let json = r#"{
        "type": "follow",
        "source": {
            "type": "user",
            "userId": "U1234567890abcdef1234567890abcdef"
        },
        "timestamp": 1704067200000,
        "mode": "active",
        "webhookEventId": "01HKTEST1234567890ABCDEFGH",
        "deliveryContext": {"isRedelivery": false},
        "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
        "follow": {"isUnblocked": false}
    }"#;
    let event: Event = serde_json::from_str(json).unwrap();
    match &event {
        Event::FollowEvent(e) => {
            assert!(!e.follow.is_unblocked);
            assert_eq!(e.reply_token, "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA");
        }
        _ => panic!("expected FollowEvent"),
    }
}

#[test]
fn deserialize_unfollow_event() {
    let json = r#"{
        "type": "unfollow",
        "source": {
            "type": "user",
            "userId": "U1234567890abcdef1234567890abcdef"
        },
        "timestamp": 1704067200000,
        "mode": "active",
        "webhookEventId": "01HKTEST1234567890ABCDEFGH",
        "deliveryContext": {"isRedelivery": false}
    }"#;
    let event: Event = serde_json::from_str(json).unwrap();
    match &event {
        Event::UnfollowEvent(e) => {
            assert_eq!(e.timestamp, 1704067200000);
        }
        _ => panic!("expected UnfollowEvent"),
    }
}

#[test]
fn deserialize_postback_event() {
    let json = r#"{
        "type": "postback",
        "source": {
            "type": "user",
            "userId": "U1234567890abcdef1234567890abcdef"
        },
        "timestamp": 1704067200000,
        "mode": "active",
        "webhookEventId": "01HKTEST1234567890ABCDEFGH",
        "deliveryContext": {"isRedelivery": false},
        "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
        "postback": {
            "data": "action=buy&itemid=123"
        }
    }"#;
    let event: Event = serde_json::from_str(json).unwrap();
    match &event {
        Event::PostbackEvent(e) => {
            assert_eq!(e.postback.data, "action=buy&itemid=123");
            assert!(e.postback.params.is_none());
        }
        _ => panic!("expected PostbackEvent"),
    }
}

#[test]
fn deserialize_postback_event_with_params() {
    let json = r#"{
        "type": "postback",
        "source": {
            "type": "user",
            "userId": "U1234567890abcdef1234567890abcdef"
        },
        "timestamp": 1704067200000,
        "mode": "active",
        "webhookEventId": "01HKTEST1234567890ABCDEFGH",
        "deliveryContext": {"isRedelivery": false},
        "postback": {
            "data": "action=buy",
            "params": {"date": "2024-01-01"}
        }
    }"#;
    let event: Event = serde_json::from_str(json).unwrap();
    match &event {
        Event::PostbackEvent(e) => {
            let params = e.postback.params.as_ref().unwrap();
            assert_eq!(params.get("date").unwrap(), "2024-01-01");
        }
        _ => panic!("expected PostbackEvent"),
    }
}

// ---------------------------------------------------------------------------
// CallbackRequest (full webhook payload)
// ---------------------------------------------------------------------------

#[test]
fn deserialize_callback_request_with_text_message() {
    let json = r#"{
        "destination": "U1234567890abcdef1234567890abcdef",
        "events": [
            {
                "type": "message",
                "source": {
                    "type": "user",
                    "userId": "Udeadbeefdeadbeefdeadbeefdeadbeef"
                },
                "timestamp": 1704067200000,
                "mode": "active",
                "webhookEventId": "01HKTEST1234567890ABCDEFGH",
                "deliveryContext": {"isRedelivery": false},
                "replyToken": "replytoken123",
                "message": {
                    "type": "text",
                    "id": "99999999999999",
                    "text": "Hello from LINE!",
                    "quoteToken": "qt123"
                }
            }
        ]
    }"#;
    let req: CallbackRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.destination, "U1234567890abcdef1234567890abcdef");
    assert_eq!(req.events.len(), 1);
    match &req.events[0] {
        Event::MessageEvent(e) => match e.message.as_ref() {
            MessageContent::TextMessageContent(t) => {
                assert_eq!(t.text, "Hello from LINE!");
            }
            _ => panic!("expected TextMessageContent"),
        },
        _ => panic!("expected MessageEvent"),
    }
}

#[test]
fn deserialize_callback_request_empty_events() {
    let json = r#"{"destination":"U1234567890abcdef1234567890abcdef","events":[]}"#;
    let req: CallbackRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.destination, "U1234567890abcdef1234567890abcdef");
    assert!(req.events.is_empty());
}

#[test]
fn deserialize_callback_request_multiple_events() {
    let json = r#"{
        "destination": "U1234567890abcdef1234567890abcdef",
        "events": [
            {
                "type": "follow",
                "source": {"type": "user", "userId": "Uaaa"},
                "timestamp": 1704067200000,
                "mode": "active",
                "webhookEventId": "01HKFOLLOW",
                "deliveryContext": {"isRedelivery": false},
                "replyToken": "rt1",
                "follow": {"isUnblocked": false}
            },
            {
                "type": "message",
                "source": {"type": "group", "groupId": "Cgroup123"},
                "timestamp": 1704067201000,
                "mode": "active",
                "webhookEventId": "01HKMESSAGE",
                "deliveryContext": {"isRedelivery": true},
                "replyToken": "rt2",
                "message": {
                    "type": "text",
                    "id": "111",
                    "text": "Hi",
                    "quoteToken": "qt"
                }
            }
        ]
    }"#;
    let req: CallbackRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.events.len(), 2);
    assert!(matches!(&req.events[0], Event::FollowEvent(_)));
    assert!(matches!(&req.events[1], Event::MessageEvent(_)));
    match &req.events[1] {
        Event::MessageEvent(e) => {
            assert!(e.delivery_context.is_redelivery);
            match e.source.as_deref() {
                Some(Source::GroupSource(g)) => {
                    assert_eq!(g.group_id, "Cgroup123");
                }
                _ => panic!("expected GroupSource"),
            }
        }
        _ => unreachable!(),
    }
}

// ---------------------------------------------------------------------------
// EventMode / DeliveryContext
// ---------------------------------------------------------------------------

#[test]
fn event_mode_roundtrip() {
    let active: EventMode = serde_json::from_str(r#""active""#).unwrap();
    assert_eq!(active, EventMode::Active);
    let standby: EventMode = serde_json::from_str(r#""standby""#).unwrap();
    assert_eq!(standby, EventMode::Standby);

    assert_eq!(
        serde_json::to_string(&EventMode::Active).unwrap(),
        r#""active""#
    );
    assert_eq!(
        serde_json::to_string(&EventMode::Standby).unwrap(),
        r#""standby""#
    );
}

#[test]
fn delivery_context_roundtrip() {
    let json = r#"{"isRedelivery":true}"#;
    let ctx: DeliveryContext = serde_json::from_str(json).unwrap();
    assert!(ctx.is_redelivery);
    let serialized = serde_json::to_string(&ctx).unwrap();
    assert_eq!(serialized, json);
}

// ---------------------------------------------------------------------------
// Standby mode event
// ---------------------------------------------------------------------------

#[test]
fn deserialize_standby_mode_event() {
    let json = r#"{
        "type": "message",
        "timestamp": 1704067200000,
        "mode": "standby",
        "webhookEventId": "01HKSTANDBY",
        "deliveryContext": {"isRedelivery": false},
        "message": {
            "type": "text",
            "id": "111",
            "text": "standby msg",
            "quoteToken": "qt"
        }
    }"#;
    let event: Event = serde_json::from_str(json).unwrap();
    match &event {
        Event::MessageEvent(e) => {
            assert_eq!(e.mode, EventMode::Standby);
            assert!(e.source.is_none());
            assert!(e.reply_token.is_none());
        }
        _ => panic!("expected MessageEvent"),
    }
}
