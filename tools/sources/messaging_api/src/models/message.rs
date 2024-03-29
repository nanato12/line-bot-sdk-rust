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

use crate::models::audio_message::AudioMessage;
use crate::models::flex_message::FlexMessage;
use crate::models::image_message::ImageMessage;
use crate::models::imagemap_message::ImagemapMessage;
use crate::models::location_message::LocationMessage;
use crate::models::sticker_message::StickerMessage;
use crate::models::template_message::TemplateMessage;
use crate::models::text_message::TextMessage;
use crate::models::video_message::VideoMessage;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "text")]
    Text(TextMessage),
    #[serde(rename = "sticker")]
    Sticker(StickerMessage),
    #[serde(rename = "image")]
    Image(ImageMessage),
    #[serde(rename = "video")]
    Video(VideoMessage),
    #[serde(rename = "audio")]
    Audio(AudioMessage),
    #[serde(rename = "location")]
    Location(LocationMessage),
    #[serde(rename = "imagemap")]
    Imagemap(ImagemapMessage),
    #[serde(rename = "template")]
    Template(TemplateMessage),
    #[serde(rename = "flex")]
    Flex(FlexMessage),
}
