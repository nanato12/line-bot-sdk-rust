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
pub struct GetMessageEventResponseClick {
    /// The URL's serial number.
    #[serde(rename = "seq", skip_serializing_if = "Option::is_none")]
    pub seq: Option<i32>,
    /// URL.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Number of times the URL was opened.
    #[serde(rename = "click", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub click: Option<Option<i64>>,
    /// Number of users that opened the URL.
    #[serde(rename = "uniqueClick", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unique_click: Option<Option<i64>>,
    /// Number of users who opened this url through any link in the message. If a message contains two links to the same URL and a user opens both links, they're counted only once.
    #[serde(rename = "uniqueClickOfRequest", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unique_click_of_request: Option<Option<i64>>,
}

impl GetMessageEventResponseClick {
    pub fn new() -> GetMessageEventResponseClick {
        GetMessageEventResponseClick {
            seq: None,
            url: None,
            click: None,
            unique_click: None,
            unique_click_of_request: None,
        }
    }
}


