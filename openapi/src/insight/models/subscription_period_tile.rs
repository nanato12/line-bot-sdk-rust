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
pub struct SubscriptionPeriodTile {
    /// Subscription period. Possible values: `within7days`, `within90days`, `unknown` etc.
    #[serde(rename = "subscriptionPeriod", skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<SubscriptionPeriodInsight>,
    /// Percentage. Possible values: [0.0,100.0] e.g. 0, 2.9, 37.6.
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl SubscriptionPeriodTile {
    pub fn new() -> SubscriptionPeriodTile {
        SubscriptionPeriodTile {
            subscription_period: None,
            percentage: None,
        }
    }
}

/// Subscription period. Possible values: `within7days`, `within90days`, `unknown` etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubscriptionPeriodInsight {
    #[serde(rename = "within7days")]
    Within7days,
    #[serde(rename = "within30days")]
    Within30days,
    #[serde(rename = "within90days")]
    Within90days,
    #[serde(rename = "within180days")]
    Within180days,
    #[serde(rename = "within365days")]
    Within365days,
    #[serde(rename = "over365days")]
    Over365days,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for SubscriptionPeriodInsight {
    fn default() -> SubscriptionPeriodInsight {
        Self::Within7days
    }
}