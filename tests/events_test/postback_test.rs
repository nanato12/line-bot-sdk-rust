#[cfg(test)]
mod postback_test {
    extern crate line_bot_sdk_rust as line;

    use line::events::postback::Params;
    use line::events::source::SouceType;
    use line::events::{EventType, Events};

    #[test]
    fn test_datetime_picker_postback_valid() {
        let string = r#"
          {
            "destination": "xxxxxxxxxx",
            "events": [
              {
                "replyToken": "b60d432864f44d079f6d8efe86cf404b",
                "type": "postback",
                "mode": "active",
                "source": {
                  "userId": "U91eeaf62d...",
                  "type": "user"
                },
                "timestamp": 1513669370317,
                "postback": {
                  "data": "storeId=12345",
                  "params": {
                    "datetime": "2017-12-25T01:00"
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
            EventType::PostBackEvent(post_back_event) => {
                // check events field value
                assert_eq!("xxxxxxxxxx", events.destination);

                // check post_back_event field value
                assert_eq!(
                    "b60d432864f44d079f6d8efe86cf404b",
                    post_back_event.reply_token
                );
                assert_eq!("active", post_back_event.mode);
                assert_eq!(1513669370317, post_back_event.timestamp);

                // check source type
                match &post_back_event.source.r#type {
                    SouceType::User(source) => {
                        assert_eq!("U91eeaf62d...", source.user_id);
                    }
                    _ => panic!("Expected SouceType::User"),
                }

                // check postback field
                assert_eq!("storeId=12345", post_back_event.postback.data);
                match &post_back_event.postback.params {
                    Some(Params::DatetimePicker { datetime, .. }) => {
                        assert_eq!(&Some(String::from("2017-12-25T01:00")), datetime);
                    }
                    _ => panic!("Expected Params::DatetimePicker"),
                }
            }
            _ => panic!("Expected EventType::AccountLinkEvent"),
        }
    }

    #[test]
    fn test_rich_menu_switch_postback_valid() {
        let string = r#"
          {
            "destination": "xxxxxxxxxx",
            "events": [
              {
                "replyToken": "b60d432864f44d079f6d8efe86cf404b",
                "type": "postback",
                "mode": "active",
                "source": {
                  "userId": "U91eeaf62d...",
                  "type": "user"
                },
                "timestamp": 1619754620404,
                "postback": {
                  "data": "richmenu-changed-to-b",
                  "params": {
                    "newRichMenuAliasId": "richmenu-alias-b",
                    "status": "SUCCESS"
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
            EventType::PostBackEvent(post_back_event) => {
                // check events field value
                assert_eq!("xxxxxxxxxx", events.destination);

                // check post_back_event field value
                assert_eq!(
                    "b60d432864f44d079f6d8efe86cf404b",
                    post_back_event.reply_token
                );
                assert_eq!("active", post_back_event.mode);
                assert_eq!(1619754620404, post_back_event.timestamp);

                // check source type
                match &post_back_event.source.r#type {
                    SouceType::User(source) => {
                        assert_eq!("U91eeaf62d...", source.user_id);
                    }
                    _ => panic!("Expected SouceType::User"),
                }

                // check postback field
                assert_eq!("richmenu-changed-to-b", post_back_event.postback.data);
                match &post_back_event.postback.params {
                    Some(Params::RichMenuSwitch {
                        new_rich_menu_alias_id,
                        status,
                    }) => {
                        assert_eq!("richmenu-alias-b", new_rich_menu_alias_id);
                        assert_eq!("SUCCESS", status);
                    }
                    _ => panic!("Expected Params::DatetimePicker"),
                }
            }
            _ => panic!("Expected EventType::AccountLinkEvent"),
        }
    }
}
