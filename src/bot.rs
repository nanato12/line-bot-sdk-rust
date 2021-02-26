//! Bot Client

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
    /// Set webhook endpoint URL [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#set-webhook-endpoint-url)
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
    /// Get webhook endpoint information [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-webhook-endpoint-information)
    /// ```
    /// let res: Result<Response, Error> = bot.get_webhook_endpoint();
    /// ```
    pub fn get_webhook_endpoint(&self) -> Result<Response, Error> {
        self.http_client
            .get("/channel/webhook/endpoint", vec![], json!({}))
    }

    /// # Note
    /// Test webhook endpoint [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#test-webhook-endpoint)
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
    /// Send reply message [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-reply-message)
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
    /// Send push message [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-push-message)
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
    /// Send multicast message [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-multicast-message)
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
    /// Send narrowcast message [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-narrowcast-message)
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
    /// Get narrowcast message status [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-narrowcast-progress-status)
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
    /// Send broadcast message [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-broadcast-message)
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
    /// Get content [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-content)
    /// ```
    /// let res: Result<Response, Error> = bot.get_content("xxxxxxx");
    /// ```
    pub fn get_content(&self, message_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/message/{messageId}/content", messageId = message_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    /// # Note
    /// Get the target limit for additional messages [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-quota)
    /// ```
    /// let res: Result<Response, Error> = bot.get_number_of_limit_additional();
    /// ```
    pub fn get_number_of_limit_additional(&self) -> Result<Response, Error> {
        self.http_client.get("/message/quota", vec![], json!({}))
    }

    /// # Note
    /// Get number of messages sent this month [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-consumption)
    /// ```
    /// let res: Result<Response, Error> = bot.get_number_of_sent_this_month();
    /// ```
    pub fn get_number_of_sent_this_month(&self) -> Result<Response, Error> {
        self.http_client
            .get("/message/quota/consumption", vec![], json!({}))
    }

    /// # Note
    /// Get number of sent reply messages [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-reply-messages)
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
    /// Get number of sent push messages [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-push-messages)
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
    /// Get number of sent multicast messages [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-multicast-messages)
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
    /// Get number of sent broadcast messages [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-number-of-broadcast-messages)
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

    // manage-audience

    // TODO: More Request Body
    // https://developers.line.biz/ja/reference/messaging-api/#create-upload-audience-group-request-body
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

    // TODO:  More Request Body
    // https://developers.line.biz/ja/reference/messaging-api/#create-upload-audience-group-by-file-request-body
    pub fn create_audience_group_for_uploading_user_ids_by_file(
        &self,
        _description: &str,
    ) -> Result<Response, Error> {
        // TODO: Fix HTTPClient post
        // File send
        self.http_client
            .post("/audienceGroup/upload/byFile", json!({}))
    }

    // TODO: Create Audience Object?
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

    // TODO: More Request Body
    // https://developers.line.biz/ja/reference/messaging-api/#update-upload-audience-group-by-file-request-body
    pub fn update_audience_group_for_uploading_user_ids_by_file(
        &self,
        _description: &str,
    ) -> Result<Response, Error> {
        // ファイルを受け取り、HTTPClientに渡す。
        // HTTPClientの改修必須
        self.http_client
            .post("/audienceGroup/upload/byFile", json!({}))
    }

    pub fn create_audience_group_for_click(
        &self,
        description: &str,
        request_id: i64,
        click_url: Option<&str>,
    ) -> Result<Response, Error> {
        let mut url: &str = "";
        match click_url {
            Some(v) => url = v,
            None => {}
        }
        let data: Value = json!(
            {
                "description": description,
                "requestId": request_id,
                "clickUrl": url
            }
        );
        self.http_client.post("/audienceGroup/click", data)
    }

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

    pub fn activate_audience(&self, audience_group_id: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/audienceGroup/{audienceGroupId}/activate",
            audienceGroupId = audience_group_id
        );
        self.http_client.put(&endpoint, json!({}))
    }

    pub fn delete_audience(&self, audience_group_id: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/audienceGroup/{audienceGroupId}",
            audienceGroupId = audience_group_id
        );
        self.http_client.delete(&endpoint, json!({}))
    }

    pub fn get_audience_information(&self, audience_group_id: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/audienceGroup/{audienceGroupId}",
            audienceGroupId = audience_group_id
        );
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#get-audience-groups
    pub fn get_many_audience_information(&self) -> Result<Response, Error> {
        // dataの処理・引数を増やす
        self.http_client
            .get("/audienceGroup/list", vec![], json!({}))
    }

    pub fn get_audience_authority_level(&self) -> Result<Response, Error> {
        self.http_client
            .get("/audienceGroup/authorityLevel", vec![], json!({}))
    }

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

    pub fn get_number_of_message_delivery(&self, date: NaiveDate) -> Result<Response, Error> {
        self.http_client.get(
            "/insight/message/delivery",
            vec![("date", &date.format("%Y%m%d").to_string())],
            json!({}),
        )
    }

    pub fn get_number_of_followes(&self, date: NaiveDate) -> Result<Response, Error> {
        self.http_client.get(
            "/bot/insight/followers",
            vec![("date", &date.format("%Y%m%d").to_string())],
            json!({}),
        )
    }

    pub fn get_friend_demographic(&self) -> Result<Response, Error> {
        self.http_client
            .get("/insight/demographic", vec![], json!({}))
    }

    pub fn get_user_interaction_statistics(&self, request_id: &str) -> Result<Response, Error> {
        self.http_client.get(
            "/insight/message/event",
            vec![("requestId", request_id)],
            json!({}),
        )
    }

    pub fn get_follower_ids(&self, continuation_token: Option<&str>) -> Result<Response, Error> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        match continuation_token {
            Some(v) => {
                &query.push(("start", v));
            }
            None => {}
        }
        self.http_client.get("/followers/ids", query, json!({}))
    }

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

    pub fn get_bot_info(&self) -> Result<Response, Error> {
        self.http_client.get("/info", vec![], json!({}))
    }

    pub fn get_group_summary(&self, group_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/group/{groupId}/summary", groupId = group_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    pub fn get_group_member_count(&self, group_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/group/{groupId}/members/count", groupId = group_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    pub fn get_group_member_ids(&self, group_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/group/{groupId}/members/ids", groupId = group_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

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

    pub fn leave_group(&self, group_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/group/{groupId}/leave", groupId = group_id);
        self.http_client.post(&endpoint, json!({}))
    }

    pub fn get_room_member_count(&self, room_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/room/{roomId}/members/count", roomId = room_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

    pub fn get_room_member_ids(&self, room_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/room/{roomId}/members/ids", roomId = room_id);
        self.http_client.get(&endpoint, vec![], json!({}))
    }

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

    pub fn leave_room(&self, room_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/room/{roomId}/leave", roomId = room_id);
        self.http_client.post(&endpoint, json!({}))
    }

    // TODO: rich-menu
    // https://developers.line.biz/ja/reference/messaging-api/#rich-menu

    pub fn issue_link_token(&self, user_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/user/{userId}/linkToken", userId = user_id);
        self.http_client.post(&endpoint, json!({}))
    }
}
