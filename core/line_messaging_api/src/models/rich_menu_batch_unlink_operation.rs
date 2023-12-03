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

/// RichMenuBatchUnlinkOperation : Unlink the rich menu for all users linked to the rich menu specified in the `from` property.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RichMenuBatchUnlinkOperation {
    /// The type of operation to the rich menu linked to the user. One of link, unlink, or unlinkAll.
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "from")]
    pub from: String,
}

impl RichMenuBatchUnlinkOperation {
    /// Unlink the rich menu for all users linked to the rich menu specified in the `from` property.
    pub fn new(r#type: String, from: String) -> RichMenuBatchUnlinkOperation {
        RichMenuBatchUnlinkOperation { r#type, from }
    }
}
