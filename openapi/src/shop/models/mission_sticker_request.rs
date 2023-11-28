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
 * Mission Stickers API
 *
 * This document describes LINE Mission Stickers API.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// MissionStickerRequest : Send mission stickers (v3)

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MissionStickerRequest {
    /// Destination user ID
    #[serde(rename = "to")]
    pub to: String,
    /// Package ID for a set of stickers
    #[serde(rename = "productId")]
    pub product_id: String,
    /// `STICKER`
    #[serde(rename = "productType")]
    pub product_type: String,
    /// `false`
    #[serde(rename = "sendPresentMessage")]
    pub send_present_message: bool,
}

impl MissionStickerRequest {
    /// Send mission stickers (v3)
    pub fn new(
        to: String,
        product_id: String,
        product_type: String,
        send_present_message: bool,
    ) -> MissionStickerRequest {
        MissionStickerRequest {
            to,
            product_id,
            product_type,
            send_present_message,
        }
    }
}
