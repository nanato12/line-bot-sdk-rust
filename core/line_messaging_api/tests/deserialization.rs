use line_messaging_api::models::{Message, TextMessage};

// ---------------------------------------------------------------------------
// Message enum
// ---------------------------------------------------------------------------

#[test]
fn deserialize_text_message() {
    let json = r#"{"type":"text","text":"Hello, world!"}"#;
    let msg: Message = serde_json::from_str(json).unwrap();
    match &msg {
        Message::TextMessage(t) => {
            assert_eq!(t.text, "Hello, world!");
            assert!(t.quick_reply.is_none());
            assert!(t.sender.is_none());
            assert!(t.emojis.is_none());
        }
        _ => panic!("expected Text"),
    }
    // roundtrip
    let serialized = serde_json::to_string(&msg).unwrap();
    let roundtrip: Message = serde_json::from_str(&serialized).unwrap();
    assert_eq!(msg, roundtrip);
}

#[test]
fn serialize_text_message() {
    let msg = Message::TextMessage(TextMessage::new("Hi!".to_string()));
    let json = serde_json::to_string(&msg).unwrap();
    // must contain type discriminator
    assert!(json.contains(r#""type":"text""#));
    assert!(json.contains(r#""text":"Hi!""#));
    // optional fields should be absent
    assert!(!json.contains("quickReply"));
    assert!(!json.contains("sender"));
    assert!(!json.contains("emojis"));
}

#[test]
fn deserialize_sticker_message() {
    let json = r#"{
        "type": "sticker",
        "packageId": "446",
        "stickerId": "1988"
    }"#;
    let msg: Message = serde_json::from_str(json).unwrap();
    match &msg {
        Message::StickerMessage(s) => {
            assert_eq!(s.package_id, "446");
            assert_eq!(s.sticker_id, "1988");
        }
        _ => panic!("expected Sticker"),
    }
}

#[test]
fn deserialize_location_message() {
    let json = r#"{
        "type": "location",
        "title": "my location",
        "address": "1-6-1 Yotsuya, Shinjuku-ku, Tokyo",
        "latitude": 35.687574,
        "longitude": 139.72922
    }"#;
    let msg: Message = serde_json::from_str(json).unwrap();
    match &msg {
        Message::LocationMessage(l) => {
            assert_eq!(l.title, "my location");
            assert_eq!(l.address, "1-6-1 Yotsuya, Shinjuku-ku, Tokyo");
            assert!((l.latitude - 35.687574).abs() < 0.0001);
            assert!((l.longitude - 139.72922).abs() < 0.0001);
        }
        _ => panic!("expected Location"),
    }
}

// ---------------------------------------------------------------------------
// Roundtrip: construct → serialize → deserialize
// ---------------------------------------------------------------------------

#[test]
fn text_message_new_roundtrip() {
    let original = TextMessage::new("test message".to_string());
    let json = serde_json::to_value(&original).unwrap();
    let deserialized: TextMessage = serde_json::from_value(json).unwrap();
    assert_eq!(original, deserialized);
}
