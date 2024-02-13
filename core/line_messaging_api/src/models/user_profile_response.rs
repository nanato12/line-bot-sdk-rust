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
pub struct UserProfileResponse {
    /// User's display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// User ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Profile image URL. `https` image URL. Not included in the response if the user doesn't have a profile image.
    #[serde(rename = "pictureUrl", skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    /// User's status message. Not included in the response if the user doesn't have a status message.
    #[serde(rename = "statusMessage", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// User's language, as a BCP 47 language tag. Not included in the response if the user hasn't yet consented to the LINE Privacy Policy.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl UserProfileResponse {
    pub fn new(display_name: String, user_id: String) -> UserProfileResponse {
        UserProfileResponse {
            display_name,
            user_id,
            picture_url: None,
            status_message: None,
            language: None,
        }
    }
}
