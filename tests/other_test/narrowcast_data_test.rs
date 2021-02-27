// This sample message
// https://developers.line.biz/ja/reference/messaging-api/#send-narrowcast-message

#[cfg(test)]
mod narrowcast_data_test {
    extern crate line_bot_sdk_rust as line;

    use line::messages::{SendMessage, SendMessageType, TextMessage};
    use line::objects::narrowcast::filter::{
        Age, AppType, Area, Demographic, Gender, Operator as DemographicOperator,
        SubscriptionPeriod,
    };
    use line::objects::narrowcast::recipient::{Audience, Operator};
    use line::objects::narrowcast::{DemographicType, Filter, Limit, Recipient, RecipientType};

    use serde_derive::Serialize;
    use serde_json::{json, Value};

    #[test]
    fn test_data_valid() {
        let expected_str: &str = r#"
            {
                "messages": [
                    {
                        "type": "text",
                        "text": "test message"
                    }
                ],
                "recipient": {
                    "type": "operator",
                    "and": [
                        {
                            "type": "audience",
                            "audienceGroupId": 5614991017776
                        },
                        {
                            "type": "operator",
                            "not": {
                                "type": "audience",
                                "audienceGroupId": 4389303728991
                            }
                        }
                    ]
                },
                "filter": {
                    "demographic": {
                        "type": "operator",
                        "or": [
                            {
                                "type": "operator",
                                "and": [
                                    {
                                        "type": "gender",
                                        "oneOf": [
                                            "male",
                                            "female"
                                        ]
                                    },
                                    {
                                        "type": "age",
                                        "gte": "age_20",
                                        "lt": "age_25"
                                    },
                                    {
                                        "type": "appType",
                                        "oneOf": [
                                            "android",
                                            "ios"
                                        ]
                                    },
                                    {
                                        "type": "area",
                                        "oneOf": [
                                            "jp_23",
                                            "jp_05"
                                        ]
                                    },
                                    {
                                        "type": "subscriptionPeriod",
                                        "gte": "day_7",
                                        "lt": "day_30"
                                    }
                                ]
                            },
                            {
                                "type": "operator",
                                "and": [
                                    {
                                        "type": "age",
                                        "gte": "age_35",
                                        "lt": "age_40"
                                    },
                                    {
                                        "type": "operator",
                                        "not": {
                                            "type": "gender",
                                            "oneOf": [
                                                "male"
                                            ]
                                        }
                                    }
                                ]
                            }
                        ]
                    }
                },
                "limit": {
                    "max": 100,
                    "upToRemainingQuota": true
                }
            }
        "#;
        let expected_json: Value = serde_json::from_str(expected_str).unwrap();

        #[derive(Serialize)]
        struct Data {
            messages: Vec<SendMessage>,
            #[serde(skip_serializing_if = "Option::is_none")]
            recipient: Option<Recipient>,
            #[serde(skip_serializing_if = "Option::is_none")]
            filter: Option<Filter>,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<Limit>,
            #[serde(
                rename = "notificationDisabled",
                skip_serializing_if = "Option::is_none"
            )]
            notification_disabled: Option<bool>,
        }

        let messages: Vec<SendMessage> = vec![SendMessage {
            r#type: SendMessageType::TextMessage(TextMessage {
                text: String::from("test message"),
                emojis: None,
            }),
        }];

        let recipient: Recipient = Recipient {
            r#type: RecipientType::Operator(Operator {
                not: None,
                or: None,
                and: Some(vec![
                    RecipientType::Audience(Audience {
                        audience_group_id: 5614991017776,
                    }),
                    RecipientType::Operator(Operator {
                        not: Some(Box::new(RecipientType::Audience(Audience {
                            audience_group_id: 4389303728991,
                        }))),
                        or: None,
                        and: None,
                    }),
                ]),
            }),
        };

        let filter: Filter = Filter {
            demographic: Demographic {
                r#type: DemographicType::Operator(DemographicOperator {
                    or: Some(vec![
                        DemographicType::Operator(DemographicOperator {
                            and: Some(vec![
                                DemographicType::Gender(Gender {
                                    one_of: vec![String::from("male"), String::from("female")],
                                }),
                                DemographicType::Age(Age {
                                    gte: Some(String::from("age_20")),
                                    lt: Some(String::from("age_25")),
                                }),
                                DemographicType::AppType(AppType {
                                    one_of: vec![String::from("android"), String::from("ios")],
                                }),
                                DemographicType::Area(Area {
                                    one_of: vec![String::from("jp_23"), String::from("jp_05")],
                                }),
                                DemographicType::SubscriptionPeriod(SubscriptionPeriod {
                                    gte: Some(String::from("day_7")),
                                    lt: Some(String::from("day_30")),
                                }),
                            ]),
                            or: None,
                            not: None,
                        }),
                        DemographicType::Operator(DemographicOperator {
                            and: Some(vec![
                                DemographicType::Age(Age {
                                    gte: Some(String::from("age_35")),
                                    lt: Some(String::from("age_40")),
                                }),
                                DemographicType::Operator(DemographicOperator {
                                    and: None,
                                    or: None,
                                    not: Some(Box::new(DemographicType::Gender(Gender {
                                        one_of: vec![String::from("male")],
                                    }))),
                                }),
                            ]),
                            or: None,
                            not: None,
                        }),
                    ]),
                    and: None,
                    not: None,
                }),
            },
        };

        let limit: Limit = Limit {
            max: 100,
            up_to_remaining_quota: true,
        };

        let data: Value = json!(Data {
            messages: messages,
            recipient: Some(recipient),
            filter: Some(filter),
            limit: Some(limit),
            notification_disabled: None
        });

        assert_eq!(data, expected_json);
    }
}
