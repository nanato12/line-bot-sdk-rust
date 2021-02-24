use crate::client::HttpClient;
use crate::events::Events;
use crate::messages::SendMessageType;
use crate::objects::Profile;
use crate::webhook;

use reqwest::blocking::Response;
use reqwest::Error;

use serde_json::json;

#[derive(Debug)]
pub struct LineBot {
    pub channel_secret: String,
    pub channel_token: String,
    pub http_client: HttpClient,
}

impl LineBot {
    pub fn new(channel_secret: &str, channel_token: &str) -> LineBot {
        LineBot {
            channel_secret: String::from(channel_secret),
            channel_token: String::from(channel_token),
            http_client: HttpClient::new(channel_token),
        }
    }

    pub fn parse_event_request(&self, signature: &str, body: &str) -> Result<Events, &'static str> {
        if webhook::validate_signature(&self.channel_secret, signature, body) {
            let res: Events = serde_json::from_str(body).expect("Failed event data parsing");
            Ok(res)
        } else {
            Err("Invalid signature value")
        }
    }

    pub fn reply_message(
        &self,
        reply_token: &str,
        msgs: Vec<SendMessageType>,
    ) -> Result<Response, Error> {
        let data = json!(
            {
                "replyToken": reply_token,
                "messages": msgs,
            }
        );
        self.http_client.post("/message/reply", data)
    }

    pub fn push_message(&self, to: &str, msgs: Vec<SendMessageType>) -> Result<Response, Error> {
        let data = json!(
            {
                "to": to,
                "messages": msgs,
            }
        );
        self.http_client.post("/message/push", data)
    }

    pub fn multicast(
        &self,
        to: Vec<String>,
        msgs: Vec<SendMessageType>,
    ) -> Result<Response, Error> {
        let data = json!(
            {
                "to": to,
                "messages": msgs,
            }
        );
        self.http_client.post("/message/multicast", data)
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#send-narrowcast-message
    // recipient, filter, limit のオブジェクトを実装する。
    pub fn narrowcast(&self, to: Vec<&str>, msgs: Vec<SendMessageType>) -> Result<Response, Error> {
        let data = json!(
            {
                "messages": msgs,
            }
        );
        self.http_client.post("/message/narrowcast", data)
    }

    pub fn get_narrowcast_progress(&self, request_id: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/message/progress/narrowcast?requestId={request_id}",
            request_id = request_id
        );
        self.http_client.get(&endpoint, json!({}))
    }

    pub fn broadcast(&self, msgs: Vec<SendMessageType>) -> Result<Response, Error> {
        let data = json!(
            {
                "messages": msgs,
            }
        );
        self.http_client.post("/message/broadcast", data)
    }

    pub fn get_content(&self, message_id: &str) -> Result<Response, Error> {
        let endpoint = format!("/message/{messageId}/content", messageId = message_id);
        self.http_client.get(&endpoint, json!({}))
    }

    pub fn get_number_of_limit_additional(&self) -> Result<Response, Error> {
        self.http_client.get("/message/quota", json!({}))
    }

    pub fn get_number_of_sent_this_month(&self) -> Result<Response, Error> {
        self.http_client
            .get("/message/quota/consumption", json!({}))
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#get-number-of-reply-messages
    // datetime型を使用する。
    pub fn get_number_of_sent_reply_messages(&self, date: &str) -> Result<Response, Error> {
        let endpoint = format!("/message/delivery/reply?date={yyyyMMdd}", yyyyMMdd = date);
        self.http_client.get(&endpoint, json!({}))
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#get-number-of-push-messages
    // datetime型を使用する。
    pub fn get_number_of_sent_push_messages(&self, date: &str) -> Result<Response, Error> {
        let endpoint = format!("/message/delivery/push?date={yyyyMMdd}", yyyyMMdd = date);
        self.http_client.get(&endpoint, json!({}))
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#get-number-of-multicast-messages
    // datetime型を使用する。
    pub fn get_number_of_sent_multicast_messages(&self, date: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/message/delivery/multicast?date={yyyyMMdd}",
            yyyyMMdd = date
        );
        self.http_client.get(&endpoint, json!({}))
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#get-number-of-broadcast-messages
    // datetime型を使用する。
    pub fn get_number_of_sent_broadcast_messages(&self, date: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/message/delivery/broadcast?date={yyyyMMdd}",
            yyyyMMdd = date
        );
        self.http_client.get(&endpoint, json!({}))
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#retry-api-request

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#manage-audience-group
    pub fn create_audience_group_for_uploading_user_ids(
        &self,
        description: &str,
    ) -> Result<Response, Error> {
        let data = json!(
            {
                "description": description,
            }
        );
        self.http_client.post("/audienceGroup/upload", data)
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#create-upload-audience-group-by-file
    pub fn create_audience_group_for_uploading_user_ids_by_file(
        &self,
        description: &str,
    ) -> Result<Response, Error> {
        // ファイルを受け取り、HTTPClientに渡す。
        // HTTPClientの改修必須
        self.http_client
            .post("/audienceGroup/upload/byFile", json!({}))
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#update-upload-audience-group
    pub fn update_audience_group_for_uploading_user_ids(
        &self,
        description: &str,
    ) -> Result<Response, Error> {
        let data = json!(
            {
                "description": description,
            }
        );
        self.http_client.post("/audienceGroup/upload", data)
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#update-upload-audience-group-by-file
    pub fn update_audience_group_for_uploading_user_ids_by_file(
        &self,
        description: &str,
    ) -> Result<Response, Error> {
        // ファイルを受け取り、HTTPClientに渡す。
        // HTTPClientの改修必須
        self.http_client
            .post("/audienceGroup/upload/byFile", json!({}))
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#create-click-audience-group
    pub fn create_audience_group_for_click(&self, description: &str) -> Result<Response, Error> {
        let data = json!(
            {
                "description": description,
            }
        );
        self.http_client.post("/audienceGroup/click", data)
    }

    pub fn create_audience_group_for_impression(
        &self,
        description: &str,
        request_id: &str,
    ) -> Result<Response, Error> {
        let data = json!(
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
        let data = json!(
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
        self.http_client.get(&endpoint, json!({}))
    }

    // TODO: https://developers.line.biz/ja/reference/messaging-api/#get-audience-groups
    pub fn get_many_audience_information(&self) -> Result<Response, Error> {
        // dataの処理・引数を増やす
        self.http_client.get("/audienceGroup/list", json!({}))
    }

    pub fn get_audience_authority_level(&self) -> Result<Response, Error> {
        self.http_client
            .get("/audienceGroup/authorityLevel", json!({}))
    }

    pub fn update_audience_authority_level(
        &self,
        authority_level: &str,
    ) -> Result<Response, Error> {
        let data = json!(
            {
                "authorityLevel": authority_level,
            }
        );
        self.http_client.put("/audienceGroup/authorityLevel", data)
    }

    // TODO: datetime型
    pub fn get_number_of_message_delivery(&self, date: &str) -> Result<Response, Error> {
        let endpoint = format!("/insight/message/delivery?date={date}", date = date);
        self.http_client.get(&endpoint, json!({}))
    }

    // TODO: datetime型
    pub fn get_number_of_followes(&self, date: &str) -> Result<Response, Error> {
        let endpoint = format!("/bot/insight/followers?date={date}", date = date);
        self.http_client.get(&endpoint, json!({}))
    }

    pub fn get_friend_demographic(&self) -> Result<Response, Error> {
        self.http_client.get("/insight/demographic", json!({}))
    }

    pub fn get_user_interaction_statistics(&self, request_id: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/insight/message/event?requestId={requestId}",
            requestId = request_id
        );
        self.http_client.get(&endpoint, json!({}))
    }

    pub fn get_follower_ids(&self, continuation_token: &str) -> Result<Response, Error> {
        let endpoint = format!(
            "/followers/ids?start={continuationToken}",
            continuationToken = continuation_token
        );
        self.http_client.get(&endpoint, json!({}))
    }

    pub fn get_bot_info(&self) -> Result<Response, Error> {
        self.http_client.get("/info", json!({}))
    }

    pub fn get_profile(&self, user_id: &str) -> Result<Profile, &str> {
        let endpoint = format!("/profile/{userId}", userId = user_id);
        match self.http_client.get(&endpoint, json!({})) {
            Ok(res) => {
                let profile: Profile = serde_json::from_str(&res.text().unwrap()).unwrap();
                Ok(profile)
            }
            Err(_) => Err("Failed get_profile"),
        }
    }
}
