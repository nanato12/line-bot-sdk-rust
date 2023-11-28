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
pub struct FlexText {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "flex", skip_serializing_if = "Option::is_none")]
    pub flex: Option<i32>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "align", skip_serializing_if = "Option::is_none")]
    pub align: Option<AlignMessagingApi>,
    #[serde(rename = "gravity", skip_serializing_if = "Option::is_none")]
    pub gravity: Option<GravityMessagingApi>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<WeightMessagingApi>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleMessagingApi>,
    #[serde(rename = "decoration", skip_serializing_if = "Option::is_none")]
    pub decoration: Option<DecorationMessagingApi>,
    #[serde(rename = "wrap", skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
    #[serde(rename = "lineSpacing", skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<String>,
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
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::messaging_api::models::Action>>,
    #[serde(rename = "maxLines", skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<i32>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<crate::messaging_api::models::FlexSpan>>,
    #[serde(rename = "adjustMode", skip_serializing_if = "Option::is_none")]
    pub adjust_mode: Option<AdjustModeMessagingApi>,
    #[serde(rename = "scaling", skip_serializing_if = "Option::is_none")]
    pub scaling: Option<bool>,
}

impl FlexText {
    pub fn new(r#type: String) -> FlexText {
        FlexText {
            r#type,
            flex: None,
            text: None,
            size: None,
            align: None,
            gravity: None,
            color: None,
            weight: None,
            style: None,
            decoration: None,
            wrap: None,
            line_spacing: None,
            margin: None,
            position: None,
            offset_top: None,
            offset_bottom: None,
            offset_start: None,
            offset_end: None,
            action: None,
            max_lines: None,
            contents: None,
            adjust_mode: None,
            scaling: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlignMessagingApi {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "center")]
    Center,
}

impl Default for AlignMessagingApi {
    fn default() -> AlignMessagingApi {
        Self::Start
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GravityMessagingApi {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "center")]
    Center,
}

impl Default for GravityMessagingApi {
    fn default() -> GravityMessagingApi {
        Self::Top
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WeightMessagingApi {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "bold")]
    Bold,
}

impl Default for WeightMessagingApi {
    fn default() -> WeightMessagingApi {
        Self::Regular
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StyleMessagingApi {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "italic")]
    Italic,
}

impl Default for StyleMessagingApi {
    fn default() -> StyleMessagingApi {
        Self::Normal
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DecorationMessagingApi {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "line-through")]
    LineThrough,
}

impl Default for DecorationMessagingApi {
    fn default() -> DecorationMessagingApi {
        Self::None
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
pub enum AdjustModeMessagingApi {
    #[serde(rename = "shrink-to-fit")]
    ShrinkToFit,
}

impl Default for AdjustModeMessagingApi {
    fn default() -> AdjustModeMessagingApi {
        Self::ShrinkToFit
    }
}

