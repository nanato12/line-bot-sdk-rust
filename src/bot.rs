use crate::client::HttpClient;
use crate::events::Events;
use crate::webhook;

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
            http_client: HttpClient::new(),
        }
    }
    // https://developers.line.biz/ja/reference/messaging-api/#signature-validation

    pub fn parse_event_request(&self, signature: &str, body: &str) -> Result<Events, &'static str> {
        if webhook::validate_signature(&self.channel_secret, signature, body) {
            // TODO: For debug, delete this sentense when release.
            println!("{}", body);
            let res: Events = serde_json::from_str(body).expect("Failed event data parsing");
            Ok(res)
        } else {
            Err("Invalid signature value")
        }
    }
}
