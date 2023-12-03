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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubscriptionPeriodDemographic {
    #[serde(rename = "day_7")]
    Variant7,
    #[serde(rename = "day_30")]
    Variant30,
    #[serde(rename = "day_90")]
    Variant90,
    #[serde(rename = "day_180")]
    Variant180,
    #[serde(rename = "day_365")]
    Variant365,
}

impl ToString for SubscriptionPeriodDemographic {
    fn to_string(&self) -> String {
        match self {
            Self::Variant7 => String::from("day_7"),
            Self::Variant30 => String::from("day_30"),
            Self::Variant90 => String::from("day_90"),
            Self::Variant180 => String::from("day_180"),
            Self::Variant365 => String::from("day_365"),
        }
    }
}

impl Default for SubscriptionPeriodDemographic {
    fn default() -> SubscriptionPeriodDemographic {
        Self::Variant7
    }
}
