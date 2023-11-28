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
pub struct PushMessageRequest {
    /// ID of the receiver.
    #[serde(rename = "to")]
    pub to: String,
    /// List of Message objects.
    #[serde(rename = "messages")]
    pub messages: Vec<crate::messaging_api::models::Message>,
    /// `true`: The user doesn’t receive a push notification when a message is sent. `false`: The user receives a push notification when the message is sent (unless they have disabled push notifications in LINE and/or their device). The default value is false. 
    #[serde(rename = "notificationDisabled", skip_serializing_if = "Option::is_none")]
    pub notification_disabled: Option<bool>,
    /// List of aggregation unit name. Case-sensitive. This functions can only be used by corporate users who have submitted the required applications. 
    #[serde(rename = "customAggregationUnits", skip_serializing_if = "Option::is_none")]
    pub custom_aggregation_units: Option<Vec<String>>,
}

impl PushMessageRequest {
    pub fn new(to: String, messages: Vec<crate::messaging_api::models::Message>) -> PushMessageRequest {
        PushMessageRequest {
            to,
            messages,
            notification_disabled: None,
            custom_aggregation_units: None,
        }
    }
}


