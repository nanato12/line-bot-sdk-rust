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
pub struct FlexVideo {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "previewUrl")]
    pub preview_url: String,
    #[serde(rename = "altContent")]
    pub alt_content: Box<crate::models::FlexComponent>,
    #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
}

impl FlexVideo {
    pub fn new(
        r#type: String,
        url: String,
        preview_url: String,
        alt_content: crate::models::FlexComponent,
    ) -> FlexVideo {
        FlexVideo {
            r#type,
            url,
            preview_url,
            alt_content: Box::new(alt_content),
            aspect_ratio: None,
            action: None,
        }
    }
}
