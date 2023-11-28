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
 * LINE Messaging API(Insight)
 *
 * This document describes LINE Messaging API(Insight).
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgeTile {
    /// users' age
    #[serde(rename = "age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeInsight>,
    /// Percentage
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl AgeTile {
    pub fn new() -> AgeTile {
        AgeTile {
            age: None,
            percentage: None,
        }
    }
}

/// users' age
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AgeInsight {
    #[serde(rename = "from0to14")]
    From0to14,
    #[serde(rename = "from15to19")]
    From15to19,
    #[serde(rename = "from20to24")]
    From20to24,
    #[serde(rename = "from25to29")]
    From25to29,
    #[serde(rename = "from30to34")]
    From30to34,
    #[serde(rename = "from35to39")]
    From35to39,
    #[serde(rename = "from40to44")]
    From40to44,
    #[serde(rename = "from45to49")]
    From45to49,
    #[serde(rename = "from50")]
    From50,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for AgeInsight {
    fn default() -> AgeInsight {
        Self::From0to14
    }
}
