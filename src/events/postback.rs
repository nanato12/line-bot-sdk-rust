use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PostBack {
    pub data: String,
    pub params: Option<PostBackParams>,
}

#[derive(Deserialize, Debug)]
pub struct PostBackParams {
    pub date: String,
    pub time: String,
    pub datetime: String,
}
