use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Source {
    #[serde(flatten)]
    pub r#type: SouceType,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SouceType {
    #[serde(rename = "user")]
    User(User),
    #[serde(rename = "group")]
    Group(Group),
    #[serde(rename = "room")]
    Room(Room),
}

#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(rename = "userId")]
    user_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Group {
    #[serde(rename = "groupId")]
    group_id: String,
    #[serde(rename = "userId")]
    user_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Room {
    #[serde(rename = "roomId")]
    room_id: String,
    #[serde(rename = "userId")]
    user_id: Option<String>,
}
