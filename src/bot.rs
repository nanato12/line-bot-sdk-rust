//! Bot Client
//! # Example
//! ```
//! extern crate line_bot_sdk_rust as line;
//! use line::bot::LineBot;
//!
//! fn main() {
//!     let bot = LineBot::new("<channel secret>", "<channel access token>");
//! }
//! ```

use crate::client::HttpClient;
use crate::events::Events;
use crate::messages::SendMessageType;
use crate::objects::narrowcast::{Filter, Limit, Recipient};
use crate::objects::Profile;
use crate::webhook;

use std::collections::HashMap;

use chrono::NaiveDate;
use reqwest::blocking::Response;
use reqwest::Error;
use serde_derive::Serialize;
use serde_json::{json, Value};

/// LineBot Client
#[derive(Debug)]
pub struct LineBot {
    pub channel_secret: String,
    pub channel_token: String,
    pub http_client: HttpClient,
}

impl LineBot {
    /// # Note
    /// Instantiate a LineBot.
    /// ```
    /// let bot = LineBot::new("<channel secret>", "<channel access token>");
    /// ```
    pub fn new(channel_secret: &str, channel_token: &str) -> LineBot {
        LineBot {
            channel_secret: String::from(channel_secret),
            channel_token: String::from(channel_token),
            http_client: HttpClient::new(channel_token),
        }
    }

    /// # Note
    /// Parse the HttpRequest content.
    /// ```
    /// let result: Result<Events, &'static str> =
    ///     bot.parse_event_request(signature, body);
    /// ```
    pub fn parse_event_request(&self, signature: &str, body: &str) -> Result<Events, &'static str> {
        if webhook::validate_signature(&self.channel_secret, signature, body) {
            let res: Events = serde_json::from_str(body).expect("Failed event data parsing");
            Ok(res)
        } else {
            Err("Invalid signature value")
        }
    }

    /// # Note
    /// Set webhook endpoint URL. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#set-webhook-endpoint-url)
    /// ```
    /// let res: Result<Response, Error> = bot.update_webhook_endpoint("https://example.com/hoge");
    /// ```
    pub fn update_webhook_endpoint(&self, endpoint: &str) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "endpoint": endpoint,
            }
        );
        self.http_client.put("/channel/webhook/endpoint", data)
    }

    /// # Note
    /// Get webhook endpoint information. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-webhook-endpoint-information)
    /// ```
    /// let res: Result<Response, Error> = bot.get_webhook_endpoint();
    /// ```
    pub fn get_webhook_endpoint(&self) -> Result<Response, Error> {
        self.http_client
            .get("/channel/webhook/endpoint", vec![], json!({}))
    }

    /// # Note
    /// Test webhook endpoint. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#test-webhook-endpoint)
    /// ```
    /// let res: Result<Response, Error> = bot.test_webhook_endpoint("https://example.com/webhook");
    /// ```
    pub fn test_webhook_endpoint(&self, endpoint: &str) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "endpoint": endpoint,
            }
        );
        self.http_client.post("/channel/webhook/test", data)
    }

    /// # Note
    /// Send reply message. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-reply-message)
    /// ```
    /// let res: Result<Response, Error> = bot.reply_message("xxxxxxxxx", vec![...]);
    /// ```
    pub fn reply_message(
        &self,
        reply_token: &str,
        msgs: Vec<SendMessageType>,
    ) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "replyToken": reply_token,
                "messages": msgs,
            }
        );
        self.http_client.post("/message/reply", data)
    }

    /// # Note
    /// Send push message. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-push-message)
    /// ```
    /// let res: Result<Response, Error> = bot.push_message("xxxxxxxxx", vec![...]);
    /// ```
    pub fn push_message(&self, to: &str, msgs: Vec<SendMessageType>) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "to": to,
                "messages": msgs,
            }
        );
        self.http_client.post("/message/push", data)
    }

    /// # Note
    /// Send multicast message. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-multicast-message)
    /// ```
    /// let res: Result<Response, Error> = bot.multicast(vec!["xxx", "yyy"], vec![...]);
    /// ```
    pub fn multicast(
        &self,
        to: Vec<String>,
        msgs: Vec<SendMessageType>,
    ) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "to": to,
                "messages": msgs,
            }
        );
        self.http_client.post("/message/multicast", data)
    }

    /// # Note
    /// Send narrowcast message. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-narrowcast-message)
    /// ```
    /// let res: Result<Response, Error> = bot.narrowcast(vec![...], Some(...), None, Some(...), Some(false));
    /// ```
    pub fn narrowcast(
        &self,
        msgs: Vec<SendMessageType>,
        recipient: Option<Recipient>,
        filter: Option<Filter>,
        limit: Option<Limit>,
        notification_disabled: Option<bool>,
    ) -> Result<Response, Error> {
        #[derive(Serialize)]
        struct Data {
            messages: Vec<SendMessageType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            recipient: Option<Recipient>,
            #[serde(skip_serializing_if = "Option::is_none")]
            filter: Option<Filter>,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<Limit>,
            #[serde(
                rename = "notificationDisabled",
                skip_serializing_if = "Option::is_none"
            )]
            notification_disabled: Option<bool>,
        }
        let data: Value = json!(Data {
            messages: msgs,
            recipient: recipient,
            filter: filter,
            limit: limit,
            notification_disabled: notification_disabled
        });
        self.http_client.post("/message/narrowcast", data)
    }

    /// # Note
    /// Get narrowcast message status. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-narrowcast-progress-status)
    /// ```
    /// let res: Result<Response, Error> = bot.get_narrowcast_progress("xxxxxxx");
    /// ```
    pub fn get_narrowcast_progress(&self, request_id: &str) -> Result<Response, Error> {
        self.http_client.get(
            "/message/progress/narrowcast",
            vec![("requestId", request_id)],
            json!({}),
        )
    }

    /// # Note
    /// Send broadcast message. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-broadcast-message)
    /// ```
    /// let res: Result<Response, Error> = bot.broadcast(vec![...]);
    /// ```
    pub fn broadcast(&self, msgs: Vec<SendMessageType>) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "messages": msgs,
            }
        );
        self.http_client.post("/message/broadcast", data)
    }

    /// # Note
    /// Get content. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-content)
    /// ```
    /// let res: Result<Response, Error> = bot.get_content("xxxxxxx");
    /// ```
    pub fn get_content(&self, message_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/message/{messageId}/content", messageId = message_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    /// # Note
    /// Get the target limit for additional messages. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-quota)
    /// ```
    /// let res: Result<Response, Error> = bot.get_number_of_limit_additional();
    /// ```
    pub fn get_number_of_limit_additional(&self) -> Result<Response, Error> {
        self.http_client.get("/message/quota", vec![], json!({}))
    }

    /// # Note
    /// Get number of messages sent this month. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-consumption)
    /// ```
    /// let res: Result<Response, Error> = bot.get_number_of_sent_this_month();
    /// ```
    pub fn get_number_of_sent_this_month(&self) -> Result<Response, Error> {
        self.http_client
            .get("/message/quota/consumption", vec![], json!({}))
    }

    /// # Note
    /// Get number of sent reply messages. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-reply-messages)
    /// ```
    /// let res: Result<Response, Error> = bot.get_number_of_sent_reply_messages(NaiveDate::from_ymd(2021, 2, 26));
    /// ```
    pub fn get_number_of_sent_reply_messages(&self, date: NaiveDate) -> Result<Response, Error> {
        self.http_client.get(
            "/message/delivery/reply",
            vec![("date", &date.format("%Y%m%d").to_string())],
            json!({}),
        )
    }

    /// # Note
    /// Get number of sent push messages. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-push-messages)
    /// ```
    /// let res: Result<Response, Error> = bot.get_number_of_sent_push_messages(NaiveDate::from_ymd(2021, 2, 26));
    /// ```
    pub fn get_number_of_sent_push_messages(&self, date: NaiveDate) -> Result<Response, Error> {
        self.http_client.get(
            "/message/delivery/push",
            vec![("date", &date.format("%Y%m%d").to_string())],
            json!({}),
        )
    }

    /// # Note
    /// Get number of sent multicast messages. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-multicast-messages)
    /// ```
    /// let res: Result<Response, Error> = bot.get_number_of_sent_multicast_messages(NaiveDate::from_ymd(2021, 2, 26));
    /// ```
    pub fn get_number_of_sent_multicast_messages(
        &self,
        date: NaiveDate,
    ) -> Result<Response, Error> {
        self.http_client.get(
            "/message/delivery/multicast",
            vec![("date", &date.format("%Y%m%d").to_string())],
            json!({}),
        )
    }

    /// # Note
    /// Get number of sent broadcast messages. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-broadcast-messages)
    /// ```
    /// let res: Result<Response, Error> = bot.get_number_of_sent_broadcast_messages(NaiveDate::from_ymd(2021, 2, 26));
    /// ```
    pub fn get_number_of_sent_broadcast_messages(
        &self,
        date: NaiveDate,
    ) -> Result<Response, Error> {
        self.http_client.get(
            "/message/delivery/broadcast",
            vec![("date", &date.format("%Y%m%d").to_string())],
            json!({}),
        )
    }

    /// # Note
    /// **TODO: Unimplemented: More Request Body <br>**
    /// You can create, update, activate, or delete an audience. Specify the audience when sending narrowcast messages. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#manage-audience-group)
    /// ```
    /// let res: Result<Response, Error> = bot.create_audience_group_for_uploading_user_ids("audienceGroupName");
    /// ```
    pub fn create_audience_group_for_uploading_user_ids(
        &self,
        description: &str,
    ) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "description": description,
            }
        );
        self.http_client.post("/audienceGroup/upload", data)
    }

    /// # Note
    /// **TODO: Unimplemented: File send <br>**
    /// Creates an audience for uploading user IDs. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#create-upload-audience-group-by-file-request-body)
    /// ```
    /// let res: Result<Response, Error> = bot.create_audience_group_for_uploading_user_ids_by_file();
    /// ```
    pub fn create_audience_group_for_uploading_user_ids_by_file(&self) -> Result<Response, Error> {
        // TODO: Fix HTTPClient post
        // File send
        self.http_client
            .post("/audienceGroup/upload/byFile", json!({}))
    }

    /// # Note
    /// **TODO: Create Audience Object <br>**
    /// Adds new user IDs or IFAs to an audience for uploading user IDs. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#update-upload-audience-group)
    /// ```
    /// let mut audience: HashMap<String, String> = HashMap::new();
    /// audience.insert("id".to_string(), "xxxxxxxxxx".to_string());
    /// let res: Result<Response, Error> = bot.update_audience_group_for_uploading_user_ids("4389303728991", "fileName", vec![audience]);
    /// ```
    pub fn update_audience_group_for_uploading_user_ids(
        &self,
        audience_group_id: i64,
        upload_description: &str,
        audiences: Vec<HashMap<String, String>>,
    ) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "audienceGroupId": audience_group_id,
                "uploadDescription": upload_description,
                "audiences": audiences
            }
        );
        self.http_client.post("/audienceGroup/upload", data)
    }

    /// # Note
    /// **TODO: Unimplemented: File send <br>**
    /// Adds new user IDs or IFAs to an audience for uploading user IDs. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#update-upload-audience-group-by-file-request-body)
    /// ```
    /// let res: Result<Response, Error> = bot.create_audience_group_for_uploading_user_ids_by_file();
    /// ```
    pub fn update_audience_group_for_uploading_user_ids_by_file(&self) -> Result<Response, Error> {
        // TODO: Fix HTTPClient post
        // File send
        self.http_client
            .post("/audienceGroup/upload/byFile", json!({}))
    }

    /// # Note
    /// Creates an audience for click-based retargeting. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#create-click-audience-group)
    /// ```
    /// let res: Result<Response, Error> = bot.create_audience_group_for_click(
    ///     "audienceGroupName",
    ///     "12222",
    ///     Some(String::from("https://line.me/en")),
    /// );
    /// ```
    pub fn create_audience_group_for_click(
        &self,
        description: &str,
        request_id: &str,
        click_url: Option<String>,
    ) -> Result<Response, Error> {
        #[derive(Serialize)]
        struct Data {
            description: String,
            #[serde(rename = "requestId")]
            request_id: String,
            #[serde(rename = "clickUrl", skip_serializing_if = "Option::is_none")]
            click_url: Option<String>,
        }
        let data: Value = json!(Data {
            description: String::from(description),
            request_id: String::from(request_id),
            click_url: click_url,
        });
        self.http_client.post("/audienceGroup/click", data)
    }

    /// # Note
    /// Creates an audience for impression-based retargeting. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#create-imp-audience-group)
    /// ```
    /// let res: Result<Response, Error> =
    ///     bot.create_audience_group_for_click("audienceGroupName", "12222");
    /// ```
    pub fn create_audience_group_for_impression(
        &self,
        description: &str,
        request_id: &str,
    ) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "description": description,
                "requestId": request_id
            }
        );
        self.http_client.post("/audienceGroup/imp", data)
    }

    /// # Note
    /// Renames an existing audience. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#set-description-audience-group)
    /// ```
    /// let res: Result<Response, Error> = bot.rename_audience("12222", "audienceGroupName");
    /// ```
    pub fn rename_audience(
        &self,
        audience_group_id: &str,
        description: &str,
    ) -> Result<Response, Error> {
        let endpoint = format!(
            "/audienceGroup/{audienceGroupId}/updateDescription",
            audienceGroupId = audience_group_id
        );
        let data: Value = json!(
            {
                "description": description,
            }
        );
        self.http_client.put(&endpoint, data)
    }

    /// # Note
    /// Activates a shared audience group. <br>
    /// An activated audience will be inactivated 180 days after activation. <br>
    /// The only audiences that need to be activated in order to use from the Messaging API are those created by LINE Ads and LINE Points Ads (Japanese only). [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#set-description-audience-group)
    /// ```
    /// let res: Result<Response, Error> = bot.activate_audience("12222");
    /// ```
    pub fn activate_audience(&self, audience_group_id: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/audienceGroup/{audienceGroupId}/activate",
            audienceGroupId = audience_group_id
        );
        self.http_client.put(&endpoint, json!({}))
    }

    /// # Note
    /// Deletes an audience. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#delete-audience-group)
    /// ```
    /// let res: Result<Response, Error> = bot.delete_audience("12222");
    /// ```
    pub fn delete_audience(&self, audience_group_id: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/audienceGroup/{audienceGroupId}",
            audienceGroupId = audience_group_id
        );
        self.http_client.delete(&endpoint, json!({}))
    }

    /// # Note
    /// Gets audience data. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-audience-group)
    /// ```
    /// let res: Result<Response, Error> = bot.get_audience_information("12222");
    /// ```
    pub fn get_audience_information(&self, audience_group_id: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/audienceGroup/{audienceGroupId}",
            audienceGroupId = audience_group_id
        );
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    /// # Note
    /// Gets data for more than one audience. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-audience-groups)
    /// ```
    /// let res: Result<Response, Error> = bot.get_many_audience_information(
    ///     "1",
    ///     Some("audienceGroupName"),
    ///     Some("40"),
    ///     None,
    ///     Some("OA_MANAGER"),
    /// );
    /// ```
    pub fn get_many_audience_information(
        &self,
        page: &str,
        description: Option<&str>,
        status: Option<&str>,
        size: Option<&str>,
        includes_external_public_groups: Option<&str>,
        create_route: Option<&str>,
    ) -> Result<Response, Error> {
        let mut query: Vec<(&str, &str)> = vec![("page", page)];
        if let Some(v) = description {
            &query.push(("description", v));
        }
        if let Some(v) = status {
            &query.push(("status", v));
        }
        if let Some(v) = size {
            &query.push(("size", v));
        }
        if let Some(v) = includes_external_public_groups {
            &query.push(("includesExternalPublicGroups", v));
        }
        if let Some(v) = create_route {
            &query.push(("createRoute", v));
        }
        self.http_client
            .get("/audienceGroup/list", query, json!({}))
    }

    /// # Note
    /// Get the authority level of the audience. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-authority-level)
    /// ```
    /// let res: Result<Response, Error> = bot.get_audience_authority_level();
    /// ```
    pub fn get_audience_authority_level(&self) -> Result<Response, Error> {
        self.http_client
            .get("/audienceGroup/authorityLevel", vec![], json!({}))
    }

    /// # Note
    /// Change the authority level of all audiences created in the same channel. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#change-authority-level)
    /// ```
    /// let res: Result<Response, Error> = bot.update_audience_authority_level("PRIVATE");
    /// ```
    pub fn update_audience_authority_level(
        &self,
        authority_level: &str,
    ) -> Result<Response, Error> {
        let data: Value = json!(
            {
                "authorityLevel": authority_level,
            }
        );
        self.http_client.put("/audienceGroup/authorityLevel", data)
    }

    /// # Note
    /// Returns the number of messages sent from LINE Official Account on a specified day. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-delivery-messages)
    /// ```
    /// let res: Result<Response, Error> =
    ///    bot.get_number_of_message_delivery(NaiveDate::from_ymd(2021, 2, 26));
    /// ```
    pub fn get_number_of_message_delivery(&self, date: NaiveDate) -> Result<Response, Error> {
        self.http_client.get(
            "/insight/message/delivery",
            vec![("date", &date.format("%Y%m%d").to_string())],
            json!({}),
        )
    }

    /// # Note
    /// Returns the number of users who have added the LINE Official Account on or before a specified date. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-followers)
    /// ```
    /// let res: Result<Response, Error> =
    ///    bot.get_number_of_followers(NaiveDate::from_ymd(2021, 2, 26));
    /// ```
    pub fn get_number_of_followers(&self, date: NaiveDate) -> Result<Response, Error> {
        self.http_client.get(
            "/bot/insight/followers",
            vec![("date", &date.format("%Y%m%d").to_string())],
            json!({}),
        )
    }

    /// # Note
    /// Retrieves the demographic attributes for a LINE Official Account's friends. <br>
    /// You can only retrieve information about friends for LINE Official Accounts created by users in Japan (JP), Thailand (TH), Taiwan (TW) and Indonesia (ID). [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-demographic)
    /// ```
    /// let res: Result<Response, Error> = bot.get_friend_demographic();
    /// ```
    pub fn get_friend_demographic(&self) -> Result<Response, Error> {
        self.http_client
            .get("/insight/demographic", vec![], json!({}))
    }

    /// # Note
    ///  Returns statistics about how users interact with narrowcast messages or broadcast messages sent from your LINE Official Account. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-message-event)
    /// ```
    /// let res: Result<Response, Error> =
    ///    bot.get_user_interaction_statistics("f70dd685-499a-4231-a441-f24b8d4fba21");
    /// ```
    pub fn get_user_interaction_statistics(&self, request_id: &str) -> Result<Response, Error> {
        self.http_client.get(
            "/insight/message/event",
            vec![("requestId", request_id)],
            json!({}),
        )
    }

    /// # Note
    /// Gets the list of User IDs of users who have added your LINE Official Account as a friend. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-follower-ids)
    /// ```
    /// let res: Result<Response, Error> = bot.get_follower_ids(Some("xxxxxxxxxxxx"));
    /// ```
    pub fn get_follower_ids(&self, continuation_token: Option<&str>) -> Result<Response, Error> {
        let mut query: Vec<(&str, &str)> = vec![];
        if let Some(v) = continuation_token {
            &query.push(("start", v));
        }
        self.http_client.get("/followers/ids", query, json!({}))
    }

    /// # Note
    /// Get the profile information of users who have added your LINE Official Account as a friend. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-profile)
    /// ```
    /// let res: Result<Profile, &str> = bot.get_profile("User ID");
    /// ```
    pub fn get_profile(&self, user_id: &str) -> Result<Profile, &str> {
        let endpoint = format!("/profile/{userId}", userId = user_id);
        match self.http_client.get(&endpoint, vec![], json!({})) {
            Ok(res) => {
                let profile: Profile = serde_json::from_str(&res.text().unwrap()).unwrap();
                Ok(profile)
            }
            Err(_) => Err("Failed get_profile"),
        }
    }

    /// # Note
    /// Gets a bot's basic information. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-bot-info)
    /// ```
    /// let res: Result<Response, Error> = bot.get_bot_info();
    /// ```
    pub fn get_bot_info(&self) -> Result<Response, Error> {
        self.http_client.get("/info", vec![], json!({}))
    }

    /// # Note
    /// Gets the group ID, group name, and group icon URL of a group where the LINE Official Account is a member. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-group-summary)
    /// ```
    /// let res: Result<Response, Error> = bot.get_group_summary("Group ID");
    /// ```
    pub fn get_group_summary(&self, group_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/group/{groupId}/summary", groupId = group_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    /// # Note
    /// Gets the count of users in a group. You can get the user in group count even if the user hasn't added the LINE Official Account as a friend or has blocked the LINE Official Account. <br>
    /// <br>
    /// The number returned excludes the LINE Official Account. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-group-summary)
    /// ```
    /// let res: Result<Response, Error> = bot.get_group_member_count("Group ID");
    /// ```
    pub fn get_group_member_count(&self, group_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/group/{groupId}/members/count", groupId = group_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    /// # Note
    /// Gets the user IDs of the members of a group that the bot is in. <br>
    /// This includes user IDs of users who have not added the LINE Official Account as a friend or has blocked the LINE Official Account. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-group-member-user-ids)
    /// ```
    /// let res: Result<Response, Error> = bot.get_group_member_ids("Group ID");
    /// ```
    pub fn get_group_member_ids(&self, group_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/group/{groupId}/members/ids", groupId = group_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    /// # Note
    /// Gets the user profile of a member of a group that the LINE Official Account is in if the user ID of the group member is known. <br>
    /// You can get user profiles of users who haven't added the LINE Official Account as a friend or have blocked the LINE Official Account. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-group-member-profile)
    /// ```
    /// let res: Result<Profile, &str> = bot.get_profile_from_group("User ID", "Group ID");
    /// ```
    pub fn get_profile_from_group(&self, user_id: &str, group_id: &str) -> Result<Profile, &str> {
        let endpoint = format!(
            "/group/{groupId}/member/{userId}",
            groupId = group_id,
            userId = user_id
        );
        match self.http_client.get(&endpoint, vec![], json!({})) {
            Ok(res) => {
                let profile: Profile = serde_json::from_str(&res.text().unwrap()).unwrap();
                Ok(profile)
            }
            Err(_) => Err("Failed get_profile_from_group"),
        }
    }

    /// # Note
    /// Leaves a group. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#leave-group)
    /// ```
    /// let res: Result<Response, Error> = bot.leave_group("Group ID");
    /// ```
    pub fn leave_group(&self, group_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/group/{groupId}/leave", groupId = group_id);
        self.http_client.post(&endpoint, json!({}))
    }

    /// # Note
    /// Gets the count of users in a room. You can get the user in room count even if the user hasn't added the LINE Official Account as a friend or has blocked the LINE Official Account. <br>
    /// The number returned excludes the LINE Official Account. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-members-room-count)
    /// ```
    /// let res: Result<Response, Error> = bot.get_room_member_count("Room ID");
    /// ```
    pub fn get_room_member_count(&self, room_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/room/{roomId}/members/count", roomId = room_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    /// # Note
    /// Gets the user IDs of the members of a room that the LINE Official Account is in. <br>
    /// This includes the user IDs of users who have not added the LINE Official Account as a friend or have blocked the LINE Official Account. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-room-member-user-ids)
    /// ```
    /// let res: Result<Response, Error> = bot.get_room_member_ids("Room ID");
    /// ```
    pub fn get_room_member_ids(&self, room_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/room/{roomId}/members/ids", roomId = room_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    /// # Note
    /// Gets the user profile of a member of a room that the LINE Official Account is in if the user ID of the room member is known. <br>
    /// You can get user profiles of users who have not added the LINE Official Account as a friend or have blocked the LINE Official Account. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-group-member-profile)
    /// ```
    /// let res: Result<Profile, &str> = bot.get_profile_from_room("User ID", "Room ID");
    /// ```
    pub fn get_profile_from_room(&self, user_id: &str, room_id: &str) -> Result<Profile, &str> {
        let endpoint = format!(
            "/room/{roomId}/member/{userId}",
            roomId = room_id,
            userId = user_id
        );
        match self.http_client.get(&endpoint, vec![], json!({})) {
            Ok(res) => {
                let profile: Profile = serde_json::from_str(&res.text().unwrap()).unwrap();
                Ok(profile)
            }
            Err(_) => Err("Failed get_profile_from_room"),
        }
    }

    /// # Note
    /// Leaves a room. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#leave-room)
    /// ```
    /// let res: Result<Response, Error> = bot.leave_room("Room ID");
    /// ```
    pub fn leave_room(&self, room_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/room/{roomId}/leave", roomId = room_id);
        self.http_client.post(&endpoint, json!({}))
    }

    // **TODO: rich-menu**
    // https://developers.line.biz/en/reference/messaging-api/#rich-menu

    /// # Note
    /// You can link the service account provided by the provider (corporate and developer) with the account of the LINE user. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#account-link)
    /// ```
    /// let res: Result<Response, Error> = bot.issue_link_token();
    /// ```
    pub fn issue_link_token(&self, user_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/user/{userId}/linkToken", userId = user_id);
        self.http_client.post(&endpoint, json!({}))
    }
}
