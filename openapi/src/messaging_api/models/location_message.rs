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
pub struct LocationMessage {
    #[serde(rename = "quickReply", skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<Box<crate::messaging_api::models::QuickReply>>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<crate::messaging_api::models::Sender>>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "latitude")]
    pub latitude: f64,
    #[serde(rename = "longitude")]
    pub longitude: f64,
}

impl LocationMessage {
    pub fn new(title: String, address: String, latitude: f64, longitude: f64) -> LocationMessage {
        LocationMessage {
            quick_reply: None,
            sender: None,
            title,
            address,
            latitude,
            longitude,
        }
    }
}
