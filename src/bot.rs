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
    // https://developers.line.biz/ja/reference/messaging-api/#signature-validation

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
