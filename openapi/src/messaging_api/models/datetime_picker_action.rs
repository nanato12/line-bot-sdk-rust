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
pub struct DatetimePickerAction {
    /// Type of action
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Label for the action.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "initial", skip_serializing_if = "Option::is_none")]
    pub initial: Option<String>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
}

impl DatetimePickerAction {
    pub fn new() -> DatetimePickerAction {
        DatetimePickerAction {
            r#type: None,
            label: None,
            data: None,
            mode: None,
            initial: None,
            max: None,
            min: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "datetime")]
    Datetime,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Date
    }
}
