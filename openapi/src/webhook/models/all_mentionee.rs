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

/// AllMentionee : Mentioned target is entire group



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllMentionee {
    /// Mentioned target.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Index position of the user mention for a character in text, with the first character being at position 0.
    #[serde(rename = "index")]
    pub index: i32,
    /// The length of the text of the mentioned user. For a mention @example, 8 is the length.
    #[serde(rename = "length")]
    pub length: i32,
}

impl AllMentionee {
    /// Mentioned target is entire group
    pub fn new(r#type: String, index: i32, length: i32) -> AllMentionee {
        AllMentionee {
            r#type,
            index,
            length,
        }
    }
}


