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
 * The version of the OpenAPI document: 0.0.2
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlexSpan {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Weight>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[serde(rename = "decoration", skip_serializing_if = "Option::is_none")]
    pub decoration: Option<Decoration>,
}

impl FlexSpan {
    pub fn new(r#type: String) -> FlexSpan {
        FlexSpan {
            r#type,
            text: None,
            size: None,
            color: None,
            weight: None,
            style: None,
            decoration: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Weight {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "bold")]
    Bold,
}

impl Default for Weight {
    fn default() -> Weight {
        Self::Regular
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "italic")]
    Italic,
}

impl Default for Style {
    fn default() -> Style {
        Self::Normal
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Decoration {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "line-through")]
    LineThrough,
}

impl Default for Decoration {
    fn default() -> Decoration {
        Self::None
    }
}
