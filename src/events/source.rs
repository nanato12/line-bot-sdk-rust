use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SouceType {
    #[serde(rename = "user")]
    User {
        #[serde(rename = "userId")]
        user_id: String,
    },
    #[serde(rename = "group")]
    Group {
        #[serde(rename = "groupId")]
        group_id: String,
        #[serde(rename = "userId")]
        user_id: String,
    },
    #[serde(rename = "room")]
    Room {
        #[serde(rename = "roomId")]
        room_id: String,
        #[serde(rename = "userId")]
        user_id: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct Source {
    #[serde(flatten)]
    pub r#type: SouceType,
}
