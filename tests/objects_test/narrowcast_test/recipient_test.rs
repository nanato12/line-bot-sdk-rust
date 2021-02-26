#[cfg(test)]
mod recipient_test {
    extern crate line_bot_sdk_rust as line;

    use line::objects::narrowcast::recipient::{Audience, Operator, Redelivery};
    use line::objects::narrowcast::{Recipient, RecipientType};

    use serde_json::{json, Value};

    #[test]
    fn test_recipient_valid() {
        let expected_str: &str = r#"
            {
                "type": "operator",
                "and": [
                    {
                        "type": "audience",
                        "audienceGroupId": 5614991017776
                    },
                    {
                        "type": "operator",
                        "not": {
                            "type": "redelivery",
                            "requestId": "5b59509c-c57b-11e9-aa8c-2a2ae2dbcce4"
                        }
                    }
                ]
            }
        "#;
        let expected_json: Value = serde_json::from_str(expected_str).unwrap();

        let redelivery: Redelivery = Redelivery {
            request_id: String::from("5b59509c-c57b-11e9-aa8c-2a2ae2dbcce4"),
        };
        let operator_rec: Operator = Operator {
            not: Some(Box::new(RecipientType::Redelivery(redelivery))),
            or: None,
            and: None,
        };
        let audience: Audience = Audience {
            audience_group_id: 5614991017776,
        };
        let operator: Operator = Operator {
            not: None,
            or: None,
            and: Some(vec![
                RecipientType::Audience(audience),
                RecipientType::Operator(operator_rec),
            ]),
        };
        let recipient: Recipient = Recipient {
            r#type: RecipientType::Operator(operator),
        };
        assert_eq!(json!(recipient), expected_json);
    }

    #[test]
    fn test_recipient_invalid() {
        let expected_str: &str = r#"
            {
                "type": "operator",
                "and": [
                    {
                        "type": "audience",
                        "audienceGroupId": 5614991017776
                    },
                    {
                        "type": "operator",
                        "not": {
                            "type": "redelivery",
                            "requestId": "5b59509c-c57b-11e9-aa8c-2a2ae2dbcce4"
                        }
                    }
                ]
            }
        "#;
        let expected_json: Value = serde_json::from_str(expected_str).unwrap();

        let redelivery: Redelivery = Redelivery {
            request_id: String::from("XXXXXXXXX"),
        };
        let operator_rec: Operator = Operator {
            not: Some(Box::new(RecipientType::Redelivery(redelivery))),
            or: None,
            and: None,
        };
        let audience: Audience = Audience {
            audience_group_id: 1234567890,
        };
        let operator: Operator = Operator {
            not: None,
            or: None,
            and: Some(vec![
                RecipientType::Audience(audience),
                RecipientType::Operator(operator_rec),
            ]),
        };
        let recipient: Recipient = Recipient {
            r#type: RecipientType::Operator(operator),
        };
        assert_ne!(json!(recipient), expected_json);
    }
}
