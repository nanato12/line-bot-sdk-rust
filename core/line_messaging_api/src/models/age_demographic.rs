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
pub enum AgeDemographic {
    #[serde(rename = "age_15")]
    Variant15,
    #[serde(rename = "age_20")]
    Variant20,
    #[serde(rename = "age_25")]
    Variant25,
    #[serde(rename = "age_30")]
    Variant30,
    #[serde(rename = "age_35")]
    Variant35,
    #[serde(rename = "age_40")]
    Variant40,
    #[serde(rename = "age_45")]
    Variant45,
    #[serde(rename = "age_50")]
    Variant50,
}

impl ToString for AgeDemographic {
    fn to_string(&self) -> String {
        match self {
            Self::Variant15 => String::from("age_15"),
            Self::Variant20 => String::from("age_20"),
            Self::Variant25 => String::from("age_25"),
            Self::Variant30 => String::from("age_30"),
            Self::Variant35 => String::from("age_35"),
            Self::Variant40 => String::from("age_40"),
            Self::Variant45 => String::from("age_45"),
            Self::Variant50 => String::from("age_50"),
        }
    }
}

impl Default for AgeDemographic {
    fn default() -> AgeDemographic {
        Self::Variant15
    }
}