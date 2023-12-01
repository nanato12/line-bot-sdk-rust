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

/// CreateClickBasedAudienceGroupResponse : Create audience for click-based retargeting

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateClickBasedAudienceGroupResponse {
    /// The audience ID.
    #[serde(rename = "audienceGroupId", skip_serializing_if = "Option::is_none")]
    pub audience_group_id: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::AudienceGroupType>,
    /// The audience's name.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the audience was created (in UNIX time).
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// The request ID that was specified when the audience was created.
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The URL that was specified when the audience was created.
    #[serde(rename = "clickUrl", skip_serializing_if = "Option::is_none")]
    pub click_url: Option<String>,
    /// How the audience was created. `MESSAGING_API`: An audience created with Messaging API.
    #[serde(rename = "createRoute", skip_serializing_if = "Option::is_none")]
    pub create_route: Option<CreateRoute>,
    /// Audience's update permission. Audiences linked to the same channel will be READ_WRITE.  - `READ`: Can use only. - `READ_WRITE`: Can use and update.
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    /// Time of audience expiration. Only returned for specific audiences.
    #[serde(rename = "expireTimestamp", skip_serializing_if = "Option::is_none")]
    pub expire_timestamp: Option<i64>,
    /// The value indicating the type of account to be sent, as specified when creating the audience for uploading user IDs. One of:  true: Accounts are specified with IFAs. false (default): Accounts are specified with user IDs.
    #[serde(rename = "isIfaAudience", skip_serializing_if = "Option::is_none")]
    pub is_ifa_audience: Option<bool>,
}

impl CreateClickBasedAudienceGroupResponse {
    /// Create audience for click-based retargeting
    pub fn new() -> CreateClickBasedAudienceGroupResponse {
        CreateClickBasedAudienceGroupResponse {
            audience_group_id: None,
            r#type: None,
            description: None,
            created: None,
            request_id: None,
            click_url: None,
            create_route: None,
            permission: None,
            expire_timestamp: None,
            is_ifa_audience: None,
        }
    }
}

/// How the audience was created. `MESSAGING_API`: An audience created with Messaging API.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateRoute {
    #[serde(rename = "MESSAGING_API")]
    MessagingApi,
}

impl Default for CreateRoute {
    fn default() -> CreateRoute {
        Self::MessagingApi
    }
}
/// Audience's update permission. Audiences linked to the same channel will be READ_WRITE.  - `READ`: Can use only. - `READ_WRITE`: Can use and update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "READ")]
    Read,
    #[serde(rename = "READ_WRITE")]
    ReadWrite,
}

impl Default for Permission {
    fn default() -> Permission {
        Self::Read
    }
}