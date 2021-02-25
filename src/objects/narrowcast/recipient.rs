use serde_derive::Serialize;

/// Please read.
/// https://developers.line.biz/ja/reference/messaging-api/#narrowcast-recipient

#[derive(Serialize, Debug)]
pub struct Recipient {
    #[serde(flatten)]
    pub r#type: RecipientType,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum RecipientType {
    #[serde(rename = "operator")]
    Operator(Operator),
    #[serde(rename = "audience")]
    Audience(Audience),
    #[serde(rename = "redelivery")]
    Redelivery(Redelivery),
}

#[derive(Serialize, Debug)]
pub struct Operator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<RecipientType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<RecipientType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<RecipientType>>,
}

#[derive(Serialize, Debug)]
pub struct Audience {
    #[serde(rename = "audienceGroupId")]
    pub audience_group_id: i64,
}

#[derive(Serialize, Debug)]
pub struct Redelivery {
    #[serde(rename = "requestId")]
    pub request_id: String,
}
