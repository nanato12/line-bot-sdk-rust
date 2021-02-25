use serde_derive::Serialize;

/// Please read.
/// https://developers.line.biz/ja/reference/messaging-api/#narrowcast-demographic-filter

#[derive(Serialize, Debug)]
pub struct Demographic {
    #[serde(flatten)]
    pub r#type: DemographicType,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum DemographicType {
    #[serde(rename = "operator")]
    Operator(Operator),
    #[serde(rename = "gender")]
    Gender(Gender),
    #[serde(rename = "age")]
    Age(Age),
    #[serde(rename = "appType")]
    AppType(AppType),
    #[serde(rename = "area")]
    Area(Area),
    #[serde(rename = "subscriptionPeriod")]
    SubscriptionPeriod(SubscriptionPeriod),
}

#[derive(Serialize, Debug)]
pub struct Operator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<DemographicType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<DemographicType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<DemographicType>>,
}

#[derive(Serialize, Debug)]
pub struct Gender {
    #[serde(rename = "oneOf")]
    pub one_of: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Age {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AppType {
    #[serde(rename = "oneOf")]
    pub one_of: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Area {
    #[serde(rename = "oneOf")]
    pub one_of: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct SubscriptionPeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,
}
