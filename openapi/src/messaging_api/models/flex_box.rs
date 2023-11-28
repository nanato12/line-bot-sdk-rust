/*
* Copyright 2023 nanato12
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

/*
 * LINE Messaging API
 *
 * This document describes LINE Messaging API.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlexBox {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "layout")]
    pub layout: LayoutMessagingApi,
    #[serde(rename = "flex", skip_serializing_if = "Option::is_none")]
    pub flex: Option<i32>,
    #[serde(rename = "contents")]
    pub contents: Vec<crate::messaging_api::models::FlexComponent>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<String>,
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionMessagingApi>,
    #[serde(rename = "offsetTop", skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<String>,
    #[serde(rename = "offsetBottom", skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<String>,
    #[serde(rename = "offsetStart", skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<String>,
    #[serde(rename = "offsetEnd", skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<String>,
    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "borderColor", skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(rename = "borderWidth", skip_serializing_if = "Option::is_none")]
    pub border_width: Option<String>,
    #[serde(rename = "cornerRadius", skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(rename = "maxWidth", skip_serializing_if = "Option::is_none")]
    pub max_width: Option<String>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "maxHeight", skip_serializing_if = "Option::is_none")]
    pub max_height: Option<String>,
    #[serde(rename = "paddingAll", skip_serializing_if = "Option::is_none")]
    pub padding_all: Option<String>,
    #[serde(rename = "paddingTop", skip_serializing_if = "Option::is_none")]
    pub padding_top: Option<String>,
    #[serde(rename = "paddingBottom", skip_serializing_if = "Option::is_none")]
    pub padding_bottom: Option<String>,
    #[serde(rename = "paddingStart", skip_serializing_if = "Option::is_none")]
    pub padding_start: Option<String>,
    #[serde(rename = "paddingEnd", skip_serializing_if = "Option::is_none")]
    pub padding_end: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::messaging_api::models::Action>>,
    #[serde(rename = "justifyContent", skip_serializing_if = "Option::is_none")]
    pub justify_content: Option<JustifyContentMessagingApi>,
    #[serde(rename = "alignItems", skip_serializing_if = "Option::is_none")]
    pub align_items: Option<AlignItemsMessagingApi>,
    #[serde(rename = "background", skip_serializing_if = "Option::is_none")]
    pub background: Option<Box<crate::messaging_api::models::FlexBoxBackground>>,
}

impl FlexBox {
    pub fn new(r#type: String, layout: LayoutMessagingApi, contents: Vec<crate::messaging_api::models::FlexComponent>) -> FlexBox {
        FlexBox {
            r#type,
            layout,
            flex: None,
            contents,
            spacing: None,
            margin: None,
            position: None,
            offset_top: None,
            offset_bottom: None,
            offset_start: None,
            offset_end: None,
            background_color: None,
            border_color: None,
            border_width: None,
            corner_radius: None,
            width: None,
            max_width: None,
            height: None,
            max_height: None,
            padding_all: None,
            padding_top: None,
            padding_bottom: None,
            padding_start: None,
            padding_end: None,
            action: None,
            justify_content: None,
            align_items: None,
            background: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LayoutMessagingApi {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "baseline")]
    Baseline,
}

impl Default for LayoutMessagingApi {
    fn default() -> LayoutMessagingApi {
        Self::Horizontal
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PositionMessagingApi {
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}

impl Default for PositionMessagingApi {
    fn default() -> PositionMessagingApi {
        Self::Relative
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JustifyContentMessagingApi {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "flex-start")]
    FlexStart,
    #[serde(rename = "flex-end")]
    FlexEnd,
    #[serde(rename = "space-between")]
    SpaceBetween,
    #[serde(rename = "space-around")]
    SpaceAround,
    #[serde(rename = "space-evenly")]
    SpaceEvenly,
}

impl Default for JustifyContentMessagingApi {
    fn default() -> JustifyContentMessagingApi {
        Self::Center
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlignItemsMessagingApi {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "flex-start")]
    FlexStart,
    #[serde(rename = "flex-end")]
    FlexEnd,
}

impl Default for AlignItemsMessagingApi {
    fn default() -> AlignItemsMessagingApi {
        Self::Center
    }
}

