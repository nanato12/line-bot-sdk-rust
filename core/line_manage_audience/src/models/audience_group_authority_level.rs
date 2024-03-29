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

/// AudienceGroupAuthorityLevel : authority level

/// authority level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AudienceGroupAuthorityLevel {
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "PRIVATE")]
    Private,
}

impl ToString for AudienceGroupAuthorityLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Public => String::from("PUBLIC"),
            Self::Private => String::from("PRIVATE"),
        }
    }
}

impl Default for AudienceGroupAuthorityLevel {
    fn default() -> AudienceGroupAuthorityLevel {
        Self::Public
    }
}
