use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct BaseSize {
    pub height: i64,
    pub width: i64,
}

#[derive(Serialize, Debug)]
pub struct Video {
    #[serde(rename = "originalContentUrl")]
    pub original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: String,
    pub area: Area,
    #[serde(rename = "externalLink")]
    pub external_link: ExternalLink,
}

#[derive(Serialize, Debug)]
pub struct Area {
    pub x: i64,
    pub y: i64,
    pub height: i64,
    pub width: i64,
}

#[derive(Serialize, Debug)]
pub struct ExternalLink {
    #[serde(rename = "linkUri")]
    pub link_uri: String,
    pub label: String,
}

#[derive(Serialize, Debug)]
pub struct Actions {
    #[serde(flatten)]
    r#type: ActionsType,
    #[serde(default)]
    label: String,
    area: Area,
}

#[derive(Serialize, Debug)]
pub enum ActionsType {
    Uri {
        #[serde(rename = "linkUri")]
        link_uri: String,
    },
    Message {
        text: String,
    },
}
