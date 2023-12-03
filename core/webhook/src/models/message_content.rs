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

use super::{
    AudioMessageContent, FileMessageContent, ImageMessageContent, LocationMessageContent,
    StickerMessageContent, TextMessageContent, VideoMessageContent,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "text")]
    TextMessageContent(TextMessageContent),
    #[serde(rename = "audio")]
    AudioMessageContent(AudioMessageContent),
    #[serde(rename = "file")]
    FileMessageContent(FileMessageContent),
    #[serde(rename = "image")]
    ImageMessageContent(ImageMessageContent),
    #[serde(rename = "location")]
    LocationMessageContent(LocationMessageContent),
    #[serde(rename = "sticker")]
    StickerMessageContent(StickerMessageContent),
    #[serde(rename = "video")]
    VideoMessageContent(VideoMessageContent),
}
