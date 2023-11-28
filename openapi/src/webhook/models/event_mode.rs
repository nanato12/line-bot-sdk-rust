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
 * Webhook Type Definition
 *
 * Webhook event definition of the LINE Messaging API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// EventMode : Channel state.

/// Channel state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventMode {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "standby")]
    Standby,
}

impl ToString for EventMode {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("active"),
            Self::Standby => String::from("standby"),
        }
    }
}

impl Default for EventMode {
    fn default() -> EventMode {
        Self::Active
    }
}
