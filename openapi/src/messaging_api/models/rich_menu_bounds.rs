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

/// RichMenuBounds : Rich menu bounds

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RichMenuBounds {
    /// Horizontal position relative to the top-left corner of the area.
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    /// Vertical position relative to the top-left corner of the area.
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
    /// Width of the area.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Height of the area.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
}

impl RichMenuBounds {
    /// Rich menu bounds
    pub fn new() -> RichMenuBounds {
        RichMenuBounds {
            x: None,
            y: None,
            width: None,
            height: None,
        }
    }
}
