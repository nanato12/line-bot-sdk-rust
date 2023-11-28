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
pub struct BotInfoResponse {
    /// Bot's user ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Bot's basic ID
    #[serde(rename = "basicId")]
    pub basic_id: String,
    /// Bot's premium ID. Not included in the response if the premium ID isn't set.
    #[serde(rename = "premiumId", skip_serializing_if = "Option::is_none")]
    pub premium_id: Option<String>,
    /// Bot's display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Profile image URL. `https` image URL. Not included in the response if the bot doesn't have a profile image.
    #[serde(rename = "pictureUrl", skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    /// Chat settings set in the LINE Official Account Manager. One of:  `chat`: Chat is set to \"On\". `bot`: Chat is set to \"Off\". 
    #[serde(rename = "chatMode")]
    pub chat_mode: ChatModeMessagingApi,
    /// Automatic read setting for messages. If the chat is set to \"Off\", auto is returned. If the chat is set to \"On\", manual is returned.  `auto`: Auto read setting is enabled. `manual`: Auto read setting is disabled.  
    #[serde(rename = "markAsReadMode")]
    pub mark_as_read_mode: MarkAsReadModeMessagingApi,
}

impl BotInfoResponse {
    pub fn new(user_id: String, basic_id: String, display_name: String, chat_mode: ChatModeMessagingApi, mark_as_read_mode: MarkAsReadModeMessagingApi) -> BotInfoResponse {
        BotInfoResponse {
            user_id,
            basic_id,
            premium_id: None,
            display_name,
            picture_url: None,
            chat_mode,
            mark_as_read_mode,
        }
    }
}

/// Chat settings set in the LINE Official Account Manager. One of:  `chat`: Chat is set to \"On\". `bot`: Chat is set to \"Off\". 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChatModeMessagingApi {
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "bot")]
    Bot,
}

impl Default for ChatModeMessagingApi {
    fn default() -> ChatModeMessagingApi {
        Self::Chat
    }
}
/// Automatic read setting for messages. If the chat is set to \"Off\", auto is returned. If the chat is set to \"On\", manual is returned.  `auto`: Auto read setting is enabled. `manual`: Auto read setting is disabled.  
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MarkAsReadModeMessagingApi {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "manual")]
    Manual,
}

impl Default for MarkAsReadModeMessagingApi {
    fn default() -> MarkAsReadModeMessagingApi {
        Self::Auto
    }
}

