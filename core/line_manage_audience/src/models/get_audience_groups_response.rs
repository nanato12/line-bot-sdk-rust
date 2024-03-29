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

/// GetAudienceGroupsResponse : Gets data for more than one audience.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAudienceGroupsResponse {
    /// An array of audience data. If there are no audiences that match the specified filter, an empty array will be returned.
    #[serde(rename = "audienceGroups", skip_serializing_if = "Option::is_none")]
    pub audience_groups: Option<Vec<crate::models::AudienceGroup>>,
    /// true when this is not the last page.
    #[serde(rename = "hasNextPage", skip_serializing_if = "Option::is_none")]
    pub has_next_page: Option<bool>,
    /// The total number of audiences that can be returned with the specified filter.
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// Of the audiences you can get with the specified filter, the number of audiences with the update permission set to READ_WRITE.
    #[serde(
        rename = "readWriteAudienceGroupTotalCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_write_audience_group_total_count: Option<i64>,
    /// The current page number.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// The maximum number of audiences on the current page.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl GetAudienceGroupsResponse {
    /// Gets data for more than one audience.
    pub fn new() -> GetAudienceGroupsResponse {
        GetAudienceGroupsResponse {
            audience_groups: None,
            has_next_page: None,
            total_count: None,
            read_write_audience_group_total_count: None,
            page: None,
            size: None,
        }
    }
}
