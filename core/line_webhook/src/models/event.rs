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
    AccountLinkEvent, ActivatedEvent, BeaconEvent, BotResumedEvent, BotSuspendedEvent,
    DeactivatedEvent, FollowEvent, JoinEvent, LeaveEvent, MemberJoinedEvent, MemberLeftEvent,
    MessageEvent, ModuleEvent, PostbackEvent, ThingsEvent, UnfollowEvent, UnsendEvent,
    VideoPlayCompleteEvent,
};

/// Event : Webhook event

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "message")]
    MessageEvent(MessageEvent),
    #[serde(rename = "unsend")]
    UnsendEvent(UnsendEvent),
    #[serde(rename = "follow")]
    FollowEvent(FollowEvent),
    #[serde(rename = "unfollow")]
    UnfollowEvent(UnfollowEvent),
    #[serde(rename = "join")]
    JoinEvent(JoinEvent),
    #[serde(rename = "leave")]
    LeaveEvent(LeaveEvent),
    #[serde(rename = "memberJoined")]
    MemberJoinedEvent(MemberJoinedEvent),
    #[serde(rename = "memberLeft")]
    MemberLeftEvent(MemberLeftEvent),
    #[serde(rename = "postback")]
    PostbackEvent(PostbackEvent),
    #[serde(rename = "videoPlayComplete")]
    VideoPlayCompleteEvent(VideoPlayCompleteEvent),
    #[serde(rename = "beacon")]
    BeaconEvent(BeaconEvent),
    #[serde(rename = "accountLink")]
    AccountLinkEvent(AccountLinkEvent),
    #[serde(rename = "things")]
    ThingsEvent(ThingsEvent),
    #[serde(rename = "module")]
    ModuleEvent(ModuleEvent),
    #[serde(rename = "activated")]
    ActivatedEvent(ActivatedEvent),
    #[serde(rename = "deactivated")]
    DeactivatedEvent(DeactivatedEvent),
    #[serde(rename = "botSuspended")]
    BotSuspendedEvent(BotSuspendedEvent),
    #[serde(rename = "botResumed")]
    BotResumedEvent(BotResumedEvent),
}
