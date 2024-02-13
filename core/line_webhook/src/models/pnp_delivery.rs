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
 * Webhook Type Definition
 *
 * Webhook event definition of the LINE Messaging API
 *
 * The version of the OpenAPI document: 1.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// PnpDelivery : A delivery object containing a hashed phone number string or a string specified by `X-Line-Delivery-Tag` header

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PnpDelivery {
    /// A hashed phone number string or a string specified by `X-Line-Delivery-Tag` header
    #[serde(rename = "data")]
    pub data: String,
}

impl PnpDelivery {
    /// A delivery object containing a hashed phone number string or a string specified by `X-Line-Delivery-Tag` header
    pub fn new(data: String) -> PnpDelivery {
        PnpDelivery { data }
    }
}
