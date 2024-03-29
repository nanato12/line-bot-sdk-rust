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
 * The version of the OpenAPI document: 0.0.2
 *
 * Generated by: https://openapi-generator.tech
 */

/// FlexSpanSize : Font size in the `size` property of the Flex span component. You can specify the size in pixels or with a keyword. FlexSpanSize just provides only keywords.

/// Font size in the `size` property of the Flex span component. You can specify the size in pixels or with a keyword. FlexSpanSize just provides only keywords.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlexSpanSize {
    #[serde(rename = "xxs")]
    Xxs,
    #[serde(rename = "xs")]
    Xs,
    #[serde(rename = "sm")]
    Sm,
    #[serde(rename = "md")]
    Md,
    #[serde(rename = "lg")]
    Lg,
    #[serde(rename = "xl")]
    Xl,
    #[serde(rename = "xxl")]
    Xxl,
    #[serde(rename = "3xl")]
    Variant3xl,
    #[serde(rename = "4xl")]
    Variant4xl,
    #[serde(rename = "5xl")]
    Variant5xl,
}

impl ToString for FlexSpanSize {
    fn to_string(&self) -> String {
        match self {
            Self::Xxs => String::from("xxs"),
            Self::Xs => String::from("xs"),
            Self::Sm => String::from("sm"),
            Self::Md => String::from("md"),
            Self::Lg => String::from("lg"),
            Self::Xl => String::from("xl"),
            Self::Xxl => String::from("xxl"),
            Self::Variant3xl => String::from("3xl"),
            Self::Variant4xl => String::from("4xl"),
            Self::Variant5xl => String::from("5xl"),
        }
    }
}

impl Default for FlexSpanSize {
    fn default() -> FlexSpanSize {
        Self::Xxs
    }
}
