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

/// AudienceGroupStatus : Status

/// Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AudienceGroupStatus {
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "ACTIVATING")]
    Activating,
}

impl ToString for AudienceGroupStatus {
    fn to_string(&self) -> String {
        match self {
            Self::InProgress => String::from("IN_PROGRESS"),
            Self::Ready => String::from("READY"),
            Self::Failed => String::from("FAILED"),
            Self::Expired => String::from("EXPIRED"),
            Self::Inactive => String::from("INACTIVE"),
            Self::Activating => String::from("ACTIVATING"),
        }
    }
}

impl Default for AudienceGroupStatus {
    fn default() -> AudienceGroupStatus {
        Self::InProgress
    }
}
