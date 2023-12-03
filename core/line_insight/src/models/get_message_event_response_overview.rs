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
 * LINE Messaging API(Insight)
 *
 * This document describes LINE Messaging API(Insight).
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetMessageEventResponseOverview : Summary of message statistics.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMessageEventResponseOverview {
    /// Request ID.
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// UNIX timestamp for message delivery time in seconds.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Number of messages delivered. This property shows values of less than 20. However, if all messages have not been sent, it will be null.
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    /// Number of users who opened the message, meaning they displayed at least 1 bubble.
    #[serde(
        rename = "uniqueImpression",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_impression: Option<Option<i64>>,
    /// Number of users who opened any URL in the message.
    #[serde(
        rename = "uniqueClick",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_click: Option<Option<i64>>,
    /// Number of users who started playing any video or audio in the message.
    #[serde(
        rename = "uniqueMediaPlayed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_media_played: Option<Option<i64>>,
    /// Number of users who played the entirety of any video or audio in the message.
    #[serde(
        rename = "uniqueMediaPlayed100Percent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_media_played100_percent: Option<Option<i64>>,
}

impl GetMessageEventResponseOverview {
    /// Summary of message statistics.
    pub fn new() -> GetMessageEventResponseOverview {
        GetMessageEventResponseOverview {
            request_id: None,
            timestamp: None,
            delivered: None,
            unique_impression: None,
            unique_click: None,
            unique_media_played: None,
            unique_media_played100_percent: None,
        }
    }
}