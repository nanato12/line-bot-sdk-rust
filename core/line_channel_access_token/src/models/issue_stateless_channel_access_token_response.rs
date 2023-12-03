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
 * Channel Access Token API
 *
 * This document describes Channel Access Token API.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// IssueStatelessChannelAccessTokenResponse : Issued stateless channel access token

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueStatelessChannelAccessTokenResponse {
    /// A stateless channel access token. The token is an opaque string which means its format is an implementation detail and the consumer of this token should never try to use the data parsed from the token.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// Duration in seconds after which the issued access token expires
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
    /// Token type. The value is always `Bearer`.
    #[serde(rename = "token_type")]
    pub token_type: String,
}

impl IssueStatelessChannelAccessTokenResponse {
    /// Issued stateless channel access token
    pub fn new(
        access_token: String,
        expires_in: i32,
        token_type: String,
    ) -> IssueStatelessChannelAccessTokenResponse {
        IssueStatelessChannelAccessTokenResponse {
            access_token,
            expires_in,
            token_type,
        }
    }
}