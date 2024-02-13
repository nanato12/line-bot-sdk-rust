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
 * The version of the OpenAPI document: 0.0.2
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAggregationUnitNameListResponse {
    /// An array of strings indicating the names of aggregation units used this month.
    #[serde(rename = "customAggregationUnits")]
    pub custom_aggregation_units: Vec<String>,
    /// A continuation token to get the next array of unit names. Returned only when there are remaining aggregation units that weren't returned in customAggregationUnits in the original request.  
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl GetAggregationUnitNameListResponse {
    pub fn new(custom_aggregation_units: Vec<String>) -> GetAggregationUnitNameListResponse {
        GetAggregationUnitNameListResponse {
            custom_aggregation_units,
            next: None,
        }
    }
}
