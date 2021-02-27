use crate::objects::Action;

use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct TemplateMessage {
    #[serde(rename = "altText")]
    pub alt_text: String,
    pub template: Template,
}

#[derive(Serialize, Debug)]
pub struct Template {
    #[serde(flatten)]
    pub r#type: TemplateType,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum TemplateType {
    #[serde(rename = "buttons")]
    Buttons {
        #[serde(rename = "thumbnailImageUrl", skip_serializing_if = "Option::is_none")]
        thumbnail_image_url: Option<String>,
        #[serde(rename = "imageAspectRatio", skip_serializing_if = "Option::is_none")]
        image_aspect_ratio: Option<String>,
        #[serde(rename = "imageSize", skip_serializing_if = "Option::is_none")]
        image_size: Option<String>,
        #[serde(
            rename = "imageBackgroundColor",
            skip_serializing_if = "Option::is_none"
        )]
        image_background_color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        text: String,
        #[serde(rename = "defaultAction", skip_serializing_if = "Option::is_none")]
        default_action: Option<Action>,
        actions: Vec<Action>,
    },
    #[serde(rename = "confirm")]
    Confirm { text: String, actions: Vec<Action> },
    #[serde(rename = "carousel")]
    Carousel {
        columns: Vec<Column>,
        #[serde(rename = "imageAspectRatio", skip_serializing_if = "Option::is_none")]
        image_aspect_ratio: Option<String>,
        #[serde(rename = "imageSize", skip_serializing_if = "Option::is_none")]
        image_size: Option<String>,
    },
    #[serde(rename = "image_carousel")]
    ImageCarousel { columns: Vec<ImageColumn> },
}

#[derive(Serialize, Debug)]
pub struct Column {
    #[serde(rename = "thumbnailImageUrl", skip_serializing_if = "Option::is_none")]
    pub thumbnail_image_url: Option<String>,
    #[serde(
        rename = "imageBackgroundColor",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub text: String,
    #[serde(rename = "defaultAction", skip_serializing_if = "Option::is_none")]
    pub default_action: Option<Action>,
    pub actions: Vec<Action>,
}

#[derive(Serialize, Debug)]
pub struct ImageColumn {
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub action: Action,
}
