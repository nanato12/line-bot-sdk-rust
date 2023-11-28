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
pub struct OperatorDemographicFilter {
    /// Type of demographic filter
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "and", skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<crate::messaging_api::models::DemographicFilter>>,
    #[serde(rename = "or", skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<crate::messaging_api::models::DemographicFilter>>,
    #[serde(rename = "not", skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<crate::messaging_api::models::DemographicFilter>>,
}

impl OperatorDemographicFilter {
    pub fn new() -> OperatorDemographicFilter {
        OperatorDemographicFilter {
            r#type: None,
            and: None,
            or: None,
            not: None,
        }
    }
}


