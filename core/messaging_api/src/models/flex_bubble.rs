/*
* Copyright (C) 2016 LINE Corp.
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
pub struct FlexBubble {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(rename = "styles", skip_serializing_if = "Option::is_none")]
    pub styles: Option<Box<crate::models::FlexBubbleStyles>>,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<crate::models::FlexBox>>,
    #[serde(rename = "hero", skip_serializing_if = "Option::is_none")]
    pub hero: Option<Box<crate::models::FlexComponent>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<crate::models::FlexBox>>,
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<Box<crate::models::FlexBox>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
}

impl FlexBubble {
    pub fn new(r#type: String) -> FlexBubble {
        FlexBubble {
            r#type,
            direction: None,
            styles: None,
            header: None,
            hero: None,
            body: None,
            footer: None,
            size: None,
            action: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "ltr")]
    Ltr,
    #[serde(rename = "rtl")]
    Rtl,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Ltr
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Size {
    #[serde(rename = "nano")]
    Nano,
    #[serde(rename = "micro")]
    Micro,
    #[serde(rename = "deca")]
    Deca,
    #[serde(rename = "hecto")]
    Hecto,
    #[serde(rename = "kilo")]
    Kilo,
    #[serde(rename = "mega")]
    Mega,
    #[serde(rename = "giga")]
    Giga,
}

impl Default for Size {
    fn default() -> Size {
        Self::Nano
    }
}
