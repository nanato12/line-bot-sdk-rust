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
pub struct TestWebhookEndpointResponse {
    /// Result of the communication from the LINE platform to the webhook URL.
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// Time of the event in milliseconds. Even in the case of a redelivered webhook, it represents the time the event occurred, not the time it was redelivered.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The HTTP status code. If the webhook response isn't received, the status code is set to zero or a negative number.
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    /// Reason for the response.
    #[serde(rename = "reason")]
    pub reason: String,
    /// Details of the response.
    #[serde(rename = "detail")]
    pub detail: String,
}

impl TestWebhookEndpointResponse {
    pub fn new(
        timestamp: String,
        status_code: i32,
        reason: String,
        detail: String,
    ) -> TestWebhookEndpointResponse {
        TestWebhookEndpointResponse {
            success: None,
            timestamp,
            status_code,
            reason,
            detail,
        }
    }
}