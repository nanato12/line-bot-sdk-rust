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

/// GetMessageContentTranscodingResponse : Transcoding response

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMessageContentTranscodingResponse {
    /// The preparation status. One of:  `processing`: Preparing to get content. `succeeded`: Ready to get the content. You can get the content sent by users. `failed`: Failed to prepare to get the content.
    #[serde(rename = "status")]
    pub status: Status,
}

impl GetMessageContentTranscodingResponse {
    /// Transcoding response
    pub fn new(status: Status) -> GetMessageContentTranscodingResponse {
        GetMessageContentTranscodingResponse { status }
    }
}

/// The preparation status. One of:  `processing`: Preparing to get content. `succeeded`: Ready to get the content. You can get the content sent by users. `failed`: Failed to prepare to get the content.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Processing
    }
}
