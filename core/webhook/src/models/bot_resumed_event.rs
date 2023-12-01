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

/// BotResumedEvent : This event indicates that the LINE Official Account has returned from the suspended state. Sent to the webhook URL server of the module channel.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotResumedEvent {
    /// Type of the event
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<crate::models::Source>>,
    /// Time of the event in milliseconds.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "mode")]
    pub mode: crate::models::EventMode,
    /// Webhook Event ID. An ID that uniquely identifies a webhook event. This is a string in ULID format.
    #[serde(rename = "webhookEventId")]
    pub webhook_event_id: String,
    #[serde(rename = "deliveryContext")]
    pub delivery_context: Box<crate::models::DeliveryContext>,
}

impl BotResumedEvent {
    /// This event indicates that the LINE Official Account has returned from the suspended state. Sent to the webhook URL server of the module channel.
    pub fn new(
        r#type: String,
        timestamp: i64,
        mode: crate::models::EventMode,
        webhook_event_id: String,
        delivery_context: crate::models::DeliveryContext,
    ) -> BotResumedEvent {
        BotResumedEvent {
            r#type,
            source: None,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context: Box::new(delivery_context),
        }
    }
}