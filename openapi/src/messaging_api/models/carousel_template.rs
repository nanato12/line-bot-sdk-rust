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
pub struct CarouselTemplate {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "columns")]
    pub columns: Vec<crate::messaging_api::models::CarouselColumn>,
    #[serde(rename = "imageAspectRatio", skip_serializing_if = "Option::is_none")]
    pub image_aspect_ratio: Option<String>,
    #[serde(rename = "imageSize", skip_serializing_if = "Option::is_none")]
    pub image_size: Option<String>,
}

impl CarouselTemplate {
    pub fn new(
        r#type: String,
        columns: Vec<crate::messaging_api::models::CarouselColumn>,
    ) -> CarouselTemplate {
        CarouselTemplate {
            r#type,
            columns,
            image_aspect_ratio: None,
            image_size: None,
        }
    }
}
