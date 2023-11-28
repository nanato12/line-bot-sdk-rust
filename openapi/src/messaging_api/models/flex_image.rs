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
pub struct FlexImage {
    #[serde(rename = "type")]
    pub r#type: String,
    /// Image URL (Max character limit: 2000) Protocol: HTTPS (TLS 1.2 or later) Image format: JPEG or PNG Maximum image size: 1024×1024 pixels Maximum file size: 10 MB (300 KB when the animated property is true)
    #[serde(rename = "url")]
    pub url: String,
    /// The ratio of the width or height of this component within the parent box.
    #[serde(rename = "flex", skip_serializing_if = "Option::is_none")]
    pub flex: Option<i32>,
    /// The minimum amount of space to include before this component in its parent container.
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    /// Reference for offsetTop, offsetBottom, offsetStart, and offsetEnd. Specify one of the following values:  `relative`: Use the previous box as reference. `absolute`: Use the top left of parent element as reference. The default value is relative.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionMessagingApi>,
    /// Offset.
    #[serde(rename = "offsetTop", skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<String>,
    /// Offset.
    #[serde(rename = "offsetBottom", skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<String>,
    /// Offset.
    #[serde(rename = "offsetStart", skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<String>,
    /// Offset.
    #[serde(rename = "offsetEnd", skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<String>,
    /// Alignment style in horizontal direction.
    #[serde(rename = "align", skip_serializing_if = "Option::is_none")]
    pub align: Option<AlignMessagingApi>,
    /// Alignment style in vertical direction.
    #[serde(rename = "gravity", skip_serializing_if = "Option::is_none")]
    pub gravity: Option<GravityMessagingApi>,
    /// The maximum image width. This is md by default.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Aspect ratio of the image. `{width}:{height}` format. Specify the value of `{width}` and `{height}` in the range from `1` to `100000`. However, you cannot set `{height}` to a value that is more than three times the value of `{width}`. The default value is `1:1`.
    #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The display style of the image if the aspect ratio of the image and that specified by the aspectRatio property do not match.
    #[serde(rename = "aspectMode", skip_serializing_if = "Option::is_none")]
    pub aspect_mode: Option<AspectModeMessagingApi>,
    /// Background color of the image. Use a hexadecimal color code.
    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::messaging_api::models::Action>>,
    /// When this is `true`, an animated image (APNG) plays. You can specify a value of true up to 10 images in a single message. You can't send messages that exceed this limit. This is `false` by default. Animated images larger than 300 KB aren't played back.
    #[serde(rename = "animated", skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

impl FlexImage {
    pub fn new(r#type: String, url: String) -> FlexImage {
        FlexImage {
            r#type,
            url,
            flex: None,
            margin: None,
            position: None,
            offset_top: None,
            offset_bottom: None,
            offset_start: None,
            offset_end: None,
            align: None,
            gravity: None,
            size: None,
            aspect_ratio: None,
            aspect_mode: None,
            background_color: None,
            action: None,
            animated: None,
        }
    }
}

/// Reference for offsetTop, offsetBottom, offsetStart, and offsetEnd. Specify one of the following values:  `relative`: Use the previous box as reference. `absolute`: Use the top left of parent element as reference. The default value is relative.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PositionMessagingApi {
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}

impl Default for PositionMessagingApi {
    fn default() -> PositionMessagingApi {
        Self::Relative
    }
}
/// Alignment style in horizontal direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlignMessagingApi {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "center")]
    Center,
}

impl Default for AlignMessagingApi {
    fn default() -> AlignMessagingApi {
        Self::Start
    }
}
/// Alignment style in vertical direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GravityMessagingApi {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "center")]
    Center,
}

impl Default for GravityMessagingApi {
    fn default() -> GravityMessagingApi {
        Self::Top
    }
}
/// The display style of the image if the aspect ratio of the image and that specified by the aspectRatio property do not match.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AspectModeMessagingApi {
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "cover")]
    Cover,
}

impl Default for AspectModeMessagingApi {
    fn default() -> AspectModeMessagingApi {
        Self::Fit
    }
}