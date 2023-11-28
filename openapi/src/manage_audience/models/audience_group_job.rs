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

/// AudienceGroupJob : Audience group job

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudienceGroupJob {
    /// A job ID.
    #[serde(rename = "audienceGroupJobId", skip_serializing_if = "Option::is_none")]
    pub audience_group_job_id: Option<i64>,
    /// An audience ID.
    #[serde(rename = "audienceGroupId", skip_serializing_if = "Option::is_none")]
    pub audience_group_id: Option<i64>,
    /// The job's description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::manage_audience::models::AudienceGroupJobType>,
    #[serde(rename = "jobStatus", skip_serializing_if = "Option::is_none")]
    pub job_status: Option<crate::manage_audience::models::AudienceGroupJobStatus>,
    #[serde(rename = "failedType", skip_serializing_if = "Option::is_none")]
    pub failed_type: Option<crate::manage_audience::models::AudienceGroupJobFailedType>,
    /// The number of accounts (recipients) that were added or removed.
    #[serde(rename = "audienceCount", skip_serializing_if = "Option::is_none")]
    pub audience_count: Option<i64>,
    /// When the job was created (in UNIX time).
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
}

impl AudienceGroupJob {
    /// Audience group job
    pub fn new() -> AudienceGroupJob {
        AudienceGroupJob {
            audience_group_job_id: None,
            audience_group_id: None,
            description: None,
            r#type: None,
            job_status: None,
            failed_type: None,
            audience_count: None,
            created: None,
        }
    }
}
