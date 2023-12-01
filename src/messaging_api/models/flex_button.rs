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
pub struct FlexButton {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "flex", skip_serializing_if = "Option::is_none")]
    pub flex: Option<i32>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[serde(rename = "action")]
    pub action: Box<crate::messaging_api::models::Action>,
    #[serde(rename = "gravity", skip_serializing_if = "Option::is_none")]
    pub gravity: Option<Gravity>,
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(rename = "offsetTop", skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<String>,
    #[serde(rename = "offsetBottom", skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<String>,
    #[serde(rename = "offsetStart", skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<String>,
    #[serde(rename = "offsetEnd", skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<String>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<Height>,
    #[serde(rename = "adjustMode", skip_serializing_if = "Option::is_none")]
    pub adjust_mode: Option<AdjustMode>,
    #[serde(rename = "scaling", skip_serializing_if = "Option::is_none")]
    pub scaling: Option<bool>,
}

impl FlexButton {
    pub fn new(r#type: String, action: crate::messaging_api::models::Action) -> FlexButton {
        FlexButton {
            r#type,
            flex: None,
            color: None,
            style: None,
            action: Box::new(action),
            gravity: None,
            margin: None,
            position: None,
            offset_top: None,
            offset_bottom: None,
            offset_start: None,
            offset_end: None,
            height: None,
            adjust_mode: None,
            scaling: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "link")]
    Link,
}

impl Default for Style {
    fn default() -> Style {
        Self::Primary
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Gravity {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "center")]
    Center,
}

impl Default for Gravity {
    fn default() -> Gravity {
        Self::Top
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Position {
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}

impl Default for Position {
    fn default() -> Position {
        Self::Relative
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Height {
    #[serde(rename = "md")]
    Md,
    #[serde(rename = "sm")]
    Sm,
}

impl Default for Height {
    fn default() -> Height {
        Self::Md
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AdjustMode {
    #[serde(rename = "shrink-to-fit")]
    ShrinkToFit,
}

impl Default for AdjustMode {
    fn default() -> AdjustMode {
        Self::ShrinkToFit
    }
}