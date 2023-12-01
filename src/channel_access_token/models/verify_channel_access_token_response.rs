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
 * Channel Access Token API
 *
 * This document describes Channel Access Token API.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// VerifyChannelAccessTokenResponse : Verification result

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyChannelAccessTokenResponse {
    /// The channel ID for which the channel access token was issued.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Number of seconds before the channel access token expires.
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    /// Permissions granted to the channel access token.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl VerifyChannelAccessTokenResponse {
    /// Verification result
    pub fn new(client_id: String, expires_in: i64) -> VerifyChannelAccessTokenResponse {
        VerifyChannelAccessTokenResponse {
            client_id,
            expires_in,
            scope: None,
        }
    }
}