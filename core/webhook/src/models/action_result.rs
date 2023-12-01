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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionResult {
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Base64-encoded binary data
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl ActionResult {
    pub fn new(r#type: Type) -> ActionResult {
        ActionResult { r#type, data: None }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "void")]
    Void,
    #[serde(rename = "binary")]
    Binary,
}

impl Default for Type {
    fn default() -> Type {
        Self::Void
    }
}