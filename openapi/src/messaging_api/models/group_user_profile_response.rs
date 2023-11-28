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
pub struct GroupUserProfileResponse {
    /// User's display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// User ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Profile image URL. `https` image URL. Not included in the response if the user doesn't have a profile image.
    #[serde(rename = "pictureUrl", skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
}

impl GroupUserProfileResponse {
    pub fn new(display_name: String, user_id: String) -> GroupUserProfileResponse {
        GroupUserProfileResponse {
            display_name,
            user_id,
            picture_url: None,
        }
    }
}


