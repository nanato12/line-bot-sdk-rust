use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Join {
    pub members: Vec<Member>,
}

#[derive(Deserialize, Debug)]
pub struct Member {
    pub r#type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
