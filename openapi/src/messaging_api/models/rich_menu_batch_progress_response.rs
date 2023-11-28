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
pub struct RichMenuBatchProgressResponse {
    #[serde(rename = "phase")]
    pub phase: crate::messaging_api::models::RichMenuBatchProgressPhase,
    /// The accepted time in milliseconds of the request of batch control the rich menu.  Format: ISO 8601 (e.g. 2023-06-08T10:15:30.121Z) Timezone: UTC
    #[serde(rename = "acceptedTime")]
    pub accepted_time: String,
    /// The completed time in milliseconds of rich menu batch control. Returned when the phase property is succeeded or failed.  Format: ISO 8601 (e.g. 2023-06-08T10:15:30.121Z) Timezone: UTC
    #[serde(rename = "completedTime", skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<String>,
}

impl RichMenuBatchProgressResponse {
    pub fn new(
        phase: crate::messaging_api::models::RichMenuBatchProgressPhase,
        accepted_time: String,
    ) -> RichMenuBatchProgressResponse {
        RichMenuBatchProgressResponse {
            phase,
            accepted_time,
            completed_time: None,
        }
    }
}
