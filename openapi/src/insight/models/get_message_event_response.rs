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
 * LINE Messaging API(Insight)
 *
 * This document describes LINE Messaging API(Insight).
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetMessageEventResponse : Statistics about how users interact with narrowcast messages or broadcast messages sent from your LINE Official Account.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMessageEventResponse {
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<Box<crate::insight::models::GetMessageEventResponseOverview>>,
    /// Array of information about individual message bubbles.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::insight::models::GetMessageEventResponseMessage>>,
    /// Array of information about opened URLs in the message.
    #[serde(rename = "clicks", skip_serializing_if = "Option::is_none")]
    pub clicks: Option<Vec<crate::insight::models::GetMessageEventResponseClick>>,
}

impl GetMessageEventResponse {
    /// Statistics about how users interact with narrowcast messages or broadcast messages sent from your LINE Official Account.
    pub fn new() -> GetMessageEventResponse {
        GetMessageEventResponse {
            overview: None,
            messages: None,
            clicks: None,
        }
    }
}
