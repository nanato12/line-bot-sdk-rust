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

/// AudienceGroupJobFailedType : Failed type

/// Failed type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AudienceGroupJobFailedType {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
    #[serde(rename = "AUDIENCE_GROUP_AUDIENCE_INSUFFICIENT")]
    AudienceGroupAudienceInsufficient,

}

impl ToString for AudienceGroupJobFailedType {
    fn to_string(&self) -> String {
        match self {
            Self::InternalError => String::from("INTERNAL_ERROR"),
            Self::AudienceGroupAudienceInsufficient => String::from("AUDIENCE_GROUP_AUDIENCE_INSUFFICIENT"),
        }
    }
}

impl Default for AudienceGroupJobFailedType {
    fn default() -> AudienceGroupJobFailedType {
        Self::InternalError
    }
}




