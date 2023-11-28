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
pub struct GetMessageEventResponseMessage {
    /// Bubble's serial number.
    #[serde(rename = "seq", skip_serializing_if = "Option::is_none")]
    pub seq: Option<i32>,
    /// Number of times the bubble was displayed.
    #[serde(
        rename = "impression",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub impression: Option<Option<i64>>,
    /// Number of times audio or video in the bubble started playing.
    #[serde(
        rename = "mediaPlayed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub media_played: Option<Option<i64>>,
    /// Number of times audio or video in the bubble started playing and was played 25% of the total time.
    #[serde(
        rename = "mediaPlayed25Percent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub media_played25_percent: Option<Option<i64>>,
    /// Number of times audio or video in the bubble started playing and was played 50% of the total time.
    #[serde(
        rename = "mediaPlayed50Percent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub media_played50_percent: Option<Option<i64>>,
    /// Number of times audio or video in the bubble started playing and was played 75% of the total time.
    #[serde(
        rename = "mediaPlayed75Percent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub media_played75_percent: Option<Option<i64>>,
    /// Number of times audio or video in the bubble started playing and was played 100% of the total time.
    #[serde(
        rename = "mediaPlayed100Percent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub media_played100_percent: Option<Option<i64>>,
    /// Number of users that started playing audio or video in the bubble.
    #[serde(
        rename = "uniqueMediaPlayed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_media_played: Option<Option<i64>>,
    /// Number of users that started playing audio or video in the bubble and played 25% of the total time.
    #[serde(
        rename = "uniqueMediaPlayed25Percent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_media_played25_percent: Option<Option<i64>>,
    /// Number of users that started playing audio or video in the bubble and played 50% of the total time.
    #[serde(
        rename = "uniqueMediaPlayed50Percent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_media_played50_percent: Option<Option<i64>>,
    /// Number of users that started playing audio or video in the bubble and played 75% of the total time.
    #[serde(
        rename = "uniqueMediaPlayed75Percent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_media_played75_percent: Option<Option<i64>>,
    /// Number of users that started playing audio or video in the bubble and played 100% of the total time.
    #[serde(
        rename = "uniqueMediaPlayed100Percent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_media_played100_percent: Option<Option<i64>>,
}

impl GetMessageEventResponseMessage {
    pub fn new() -> GetMessageEventResponseMessage {
        GetMessageEventResponseMessage {
            seq: None,
            impression: None,
            media_played: None,
            media_played25_percent: None,
            media_played50_percent: None,
            media_played75_percent: None,
            media_played100_percent: None,
            unique_media_played: None,
            unique_media_played25_percent: None,
            unique_media_played50_percent: None,
            unique_media_played75_percent: None,
            unique_media_played100_percent: None,
        }
    }
}
