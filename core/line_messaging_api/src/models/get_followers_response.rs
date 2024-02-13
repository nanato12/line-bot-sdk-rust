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
pub struct GetFollowersResponse {
    /// An array of strings indicating user IDs of users that have added the LINE Official Account as a friend. Only users of LINE for iOS and LINE for Android are included in `userIds`.
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
    /// A continuation token to get the next array of user IDs. Returned only when there are remaining user IDs that weren't returned in `userIds` in the original request. The number of user IDs in the `userIds` element doesn't have to reach the maximum number specified by `limit` for the `next` property to be included in the response.  
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl GetFollowersResponse {
    pub fn new(user_ids: Vec<String>) -> GetFollowersResponse {
        GetFollowersResponse {
            user_ids,
            next: None,
        }
    }
}
