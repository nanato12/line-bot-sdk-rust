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

use crate::messaging_api::models::audio_message::AudioMessage;
use crate::messaging_api::models::flex_message::FlexMessage;
use crate::messaging_api::models::image_message::ImageMessage;
use crate::messaging_api::models::imagemap_message::ImagemapMessage;
use crate::messaging_api::models::location_message::LocationMessage;
use crate::messaging_api::models::sticker_message::StickerMessage;
use crate::messaging_api::models::template_message::TemplateMessage;
use crate::messaging_api::models::text_message::TextMessage;
use crate::messaging_api::models::video_message::VideoMessage;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "")]
pub enum Message {
    Text(TextMessage),
    Sticker(StickerMessage),
    Image(ImageMessage),
    Video(VideoMessage),
    Audio(AudioMessage),
    Location(LocationMessage),
    Imagemap(ImagemapMessage),
    Template(TemplateMessage),
    Flex(FlexMessage),
}
