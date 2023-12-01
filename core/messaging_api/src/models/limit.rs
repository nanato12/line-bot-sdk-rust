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

/// Limit : Limit of the Narrowcast

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Limit {
    /// The maximum number of narrowcast messages to send. Use this parameter to limit the number of narrowcast messages sent. The recipients will be chosen at random.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// If true, the message will be sent within the maximum number of deliverable messages. The default value is `false`.  Targets will be selected at random.
    #[serde(rename = "upToRemainingQuota", skip_serializing_if = "Option::is_none")]
    pub up_to_remaining_quota: Option<bool>,
}

impl Limit {
    /// Limit of the Narrowcast
    pub fn new() -> Limit {
        Limit {
            max: None,
            up_to_remaining_quota: None,
        }
    }
}