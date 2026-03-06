use line_messaging_api::models::{
    flex_box::Layout, FlexBox, FlexBubble, FlexComponent, FlexContainer, Message, TextMessage,
};

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
// Direct serialization of discriminator children
// ---------------------------------------------------------------------------

#[test]
fn flex_box_direct_serialize_includes_type() {
    let flex_box = FlexBox::new(Layout::Vertical, vec![]);
    let json = serde_json::to_string(&flex_box).unwrap();
    assert!(json.contains(r#""type":"box""#));
    assert!(json.contains(r#""layout":"vertical""#));
}

#[test]
fn flex_bubble_direct_serialize_includes_type() {
    let bubble = FlexBubble::new();
    let json = serde_json::to_string(&bubble).unwrap();
    assert!(json.contains(r#""type":"bubble""#));
}

#[test]
fn flex_container_deserialize_roundtrip_no_duplicate_type() {
    // Deserialized structs have empty r#type (skip_deserializing), so
    // re-serialization through the tagged enum produces only one "type" key.
    let json = r#"{"type":"bubble","size":"mega"}"#;
    let container: FlexContainer = serde_json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&container).unwrap();
    assert_eq!(serialized.matches(r#""type""#).count(), 1);
    let roundtrip: FlexContainer = serde_json::from_str(&serialized).unwrap();
    assert_eq!(container, roundtrip);
}

#[test]
fn flex_component_deserialize_roundtrip_no_duplicate_type() {
    let json = r#"{"type":"box","layout":"horizontal","contents":[]}"#;
    let component: FlexComponent = serde_json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&component).unwrap();
    assert_eq!(serialized.matches(r#""type""#).count(), 1);
    let roundtrip: FlexComponent = serde_json::from_str(&serialized).unwrap();
    assert_eq!(component, roundtrip);
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
