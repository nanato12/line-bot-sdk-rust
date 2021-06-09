#[cfg(test)]
mod account_link_test {
    extern crate line_bot_sdk_rust as line;

    use line::events::source::SourceType;
    use line::events::{EventType, Events};

    #[test]
    fn test_parse_valid() {
        let string = r#"
          {
            "destination": "xxxxxxxxxx",
            "events": [
              {
                "replyToken": "b60d432864f44d079f6d8efe86cf404b",
                "type": "accountLink",
                "mode": "active",
                "source": {
                  "userId": "U91eeaf62d...",
                  "type": "user"
                },
                "timestamp": 1513669370317,
                "link": {
                  "result": "ok",
                  "nonce": "xxxxxxxxxxxxxxx"
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
            EventType::AccountLinkEvent(account_link_event) => {
                // check events field value
                assert_eq!("xxxxxxxxxx", events.destination);

                // check account_link_event field value
                assert_eq!(
                    "b60d432864f44d079f6d8efe86cf404b",
                    account_link_event.reply_token
                );
                assert_eq!("active", account_link_event.mode);
                assert_eq!(1513669370317, account_link_event.timestamp);

                // check source type
                match &account_link_event.source.r#type {
                    SourceType::User(source) => {
                        assert_eq!("U91eeaf62d...", source.user_id);
                    }
                    _ => panic!("Expected SourceType::User"),
                }

                // check link field
                assert_eq!("ok", account_link_event.link.result);
                assert_eq!("xxxxxxxxxxxxxxx", account_link_event.link.nonce);
            }
            _ => panic!("Expected EventType::AccountLinkEvent"),
        }
    }
}
