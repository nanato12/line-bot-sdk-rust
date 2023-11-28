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

/// CreateClickBasedAudienceGroupRequest : Create audience for click-based retargeting



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateClickBasedAudienceGroupRequest {
    /// The audience's name. This is case-insensitive, meaning AUDIENCE and audience are considered identical. Max character limit: 120 
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The request ID of a broadcast or narrowcast message sent in the past 60 days. Each Messaging API request has a request ID. 
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The URL clicked by the user. If empty, users who clicked any URL in the message are added to the list of recipients. Max character limit: 2,000 
    #[serde(rename = "clickUrl", skip_serializing_if = "Option::is_none")]
    pub click_url: Option<String>,
}

impl CreateClickBasedAudienceGroupRequest {
    /// Create audience for click-based retargeting
    pub fn new() -> CreateClickBasedAudienceGroupRequest {
        CreateClickBasedAudienceGroupRequest {
            description: None,
            request_id: None,
            click_url: None,
        }
    }
}


