use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Left {
    pub members: Vec<Member>,
}

#[derive(Deserialize, Debug)]
pub struct Member {
    pub r#type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
