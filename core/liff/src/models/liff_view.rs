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
 * LIFF server API
 *
 * LIFF Server API.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiffView {
    /// Size of the LIFF app view. Specify one of these values: - compact - tall - full
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Endpoint URL. This is the URL of the web app that implements the LIFF app (e.g. https://example.com). Used when the LIFF app is launched using the LIFF URL. The URL scheme must be https. URL fragments (#URL-fragment) can't be specified.
    #[serde(rename = "url")]
    pub url: String,
    /// `true` to use the LIFF app in modular mode. When in modular mode, the action button in the header is not displayed.
    #[serde(rename = "moduleMode", skip_serializing_if = "Option::is_none")]
    pub module_mode: Option<bool>,
}

impl LiffView {
    pub fn new(r#type: Type, url: String) -> LiffView {
        LiffView {
            r#type,
            url,
            module_mode: None,
        }
    }
}

/// Size of the LIFF app view. Specify one of these values: - compact - tall - full
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "tall")]
    Tall,
    #[serde(rename = "full")]
    Full,
}

impl Default for Type {
    fn default() -> Type {
        Self::Compact
    }
}
