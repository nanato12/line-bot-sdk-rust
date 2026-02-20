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

//! LINE API client module.
//!
//! Provides the [`LINE`] struct, which bundles all LINE API clients into a single entry point.

use std::sync::Arc;
use std::time::Duration;

use hyper_rustls::HttpsConnector;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use line_channel_access_token::apis::{
    configuration::Configuration as ChannelAccessTokenApiConfiguration, ChannelAccessTokenApiClient,
};
use line_insight::apis::{configuration::Configuration as InsightConfiguration, InsightApiClient};
use line_liff::apis::{configuration::Configuration as LiffConfiguration, LiffApiClient};
use line_manage_audience::apis::{
    configuration::Configuration as ManageAudienceConfiguration, ManageAudienceApiClient,
    ManageAudienceBlobApiClient,
};
use line_messaging_api::apis::{
    configuration::Configuration as MessagingApiConfiguration, MessagingApiApiClient,
    MessagingApiBlobApiClient,
};
use line_module::apis::{
    configuration::Configuration as LineModuleConfiguration, LineModuleApiClient,
};
use line_module_attach::apis::{
    configuration::Configuration as LineModuleAttachConfiguration, LineModuleAttachApiClient,
};
use line_shop::apis::{configuration::Configuration as ShopConfiguration, ShopApiClient};
use line_webhook::apis::{
    configuration::Configuration as WebhookConfiguration, DummyApiClient as WebhookDummyApiClient,
};

type HttpsClient = HttpsConnector<HttpConnector>;

/// A unified client for all LINE Platform APIs.
///
/// `LINE` bundles all API-specific clients, each configured with the same channel access token
/// and sharing a single HTTPS connection pool via `hyper`.
///
/// # Example
///
/// ```no_run
/// use line_bot_sdk_rust::client::LINE;
/// use line_bot_sdk_rust::line_messaging_api::apis::MessagingApiApi;
/// use line_bot_sdk_rust::line_messaging_api::models::{
///     Message, ReplyMessageRequest, TextMessage,
/// };
///
/// # async fn example() {
/// let line = LINE::new("YOUR_CHANNEL_ACCESS_TOKEN".to_string());
///
/// let req = ReplyMessageRequest {
///     reply_token: "reply_token".to_string(),
///     messages: vec![Message::Text(TextMessage::new("Hello!".to_string()))],
///     notification_disabled: Some(false),
/// };
/// let _ = line.messaging_api_client.reply_message(req).await;
/// # }
/// ```
#[derive(Clone)]
pub struct LINE {
    pub channel_access_token_api_client: ChannelAccessTokenApiClient<HttpsClient>,
    pub insight_api_client: InsightApiClient<HttpsClient>,
    pub liff_api_client: LiffApiClient<HttpsClient>,
    pub manage_audience_api_client: ManageAudienceApiClient<HttpsClient>,
    pub manage_audience_blob_api_client: ManageAudienceBlobApiClient<HttpsClient>,
    pub messaging_api_client: MessagingApiApiClient<HttpsClient>,
    pub messaging_api_blob_client: MessagingApiBlobApiClient<HttpsClient>,
    pub module_api_client: LineModuleApiClient<HttpsClient>,
    pub module_attach_api_client: LineModuleAttachApiClient<HttpsClient>,
    pub shop_api_client: ShopApiClient<HttpsClient>,
    pub webhook_dummy_api_client: WebhookDummyApiClient<HttpsClient>,
}

impl LINE {
    /// Creates a new `LINE` client with the given channel access token.
    ///
    /// All API-specific clients are initialized with the same token and share a single
    /// HTTPS connection via `hyper` + `hyper-rustls`.
    pub fn new(token: String) -> LINE {
        LINEBuilder {
            token,
            timeout: None,
        }
        .build()
    }

    /// Returns a [`LINEBuilder`] for constructing a `LINE` client with custom settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use line_bot_sdk_rust::client::LINE;
    ///
    /// let line = LINE::builder("YOUR_CHANNEL_ACCESS_TOKEN".to_string())
    ///     .timeout(Duration::from_secs(30))
    ///     .build();
    /// ```
    pub fn builder(token: String) -> LINEBuilder {
        LINEBuilder {
            token,
            timeout: None,
        }
    }

    fn create_hyper_client() -> Client<HttpsClient, String> {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("no native root certs found")
            .https_only()
            .enable_http1()
            .build();
        Client::builder(TokioExecutor::new()).build(https)
    }
}

/// Builder for configuring a [`LINE`] client.
///
/// Obtained via [`LINE::builder`].
pub struct LINEBuilder {
    token: String,
    timeout: Option<Duration>,
}

impl LINEBuilder {
    /// Sets the request timeout for all API calls.
    ///
    /// When set, each HTTP request will fail with a timeout error if it does not
    /// complete within the specified duration. By default there is no timeout.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Builds the [`LINE`] client with the configured settings.
    pub fn build(self) -> LINE {
        let client = LINE::create_hyper_client();
        let timeout = self.timeout;
        let token = self.token;

        // channel_access_token_api
        let mut channel_access_token_api_conf =
            ChannelAccessTokenApiConfiguration::with_client(client.clone());
        channel_access_token_api_conf.oauth_access_token = Some(token.to_owned());
        channel_access_token_api_conf.timeout = timeout;
        let channel_access_token_api_client =
            ChannelAccessTokenApiClient::new(Arc::new(channel_access_token_api_conf));

        // insight
        let mut insight_conf = InsightConfiguration::with_client(client.clone());
        insight_conf.oauth_access_token = Some(token.to_owned());
        insight_conf.timeout = timeout;
        let insight_api_client = InsightApiClient::new(Arc::new(insight_conf));

        // liff
        let mut liff_conf = LiffConfiguration::with_client(client.clone());
        liff_conf.oauth_access_token = Some(token.to_owned());
        liff_conf.timeout = timeout;
        let liff_api_client = LiffApiClient::new(Arc::new(liff_conf));

        // manage_audience
        let mut manage_audience_conf = ManageAudienceConfiguration::with_client(client.clone());
        manage_audience_conf.oauth_access_token = Some(token.to_owned());
        manage_audience_conf.timeout = timeout;
        let manage_audience_api_client =
            ManageAudienceApiClient::new(Arc::new(manage_audience_conf));

        // manage_audience_blob
        let mut manage_audience_blob_conf =
            ManageAudienceConfiguration::with_client(client.clone());
        manage_audience_blob_conf.oauth_access_token = Some(token.to_owned());
        manage_audience_blob_conf.timeout = timeout;
        let manage_audience_blob_api_client =
            ManageAudienceBlobApiClient::new(Arc::new(manage_audience_blob_conf));

        // messaging_api
        let mut messaging_api_conf = MessagingApiConfiguration::with_client(client.clone());
        messaging_api_conf.oauth_access_token = Some(token.to_owned());
        messaging_api_conf.timeout = timeout;
        let messaging_api_client = MessagingApiApiClient::new(Arc::new(messaging_api_conf));

        // messaging_api_blob
        let mut messaging_api_blob_conf = MessagingApiConfiguration::with_client(client.clone());
        messaging_api_blob_conf.oauth_access_token = Some(token.to_owned());
        messaging_api_blob_conf.timeout = timeout;
        let messaging_api_blob_client =
            MessagingApiBlobApiClient::new(Arc::new(messaging_api_blob_conf));

        // module
        let mut module_conf = LineModuleConfiguration::with_client(client.clone());
        module_conf.oauth_access_token = Some(token.to_owned());
        module_conf.timeout = timeout;
        let module_api_client = LineModuleApiClient::new(Arc::new(module_conf));

        // module_attach
        let mut module_attach_conf = LineModuleAttachConfiguration::with_client(client.clone());
        module_attach_conf.oauth_access_token = Some(token.to_owned());
        module_attach_conf.timeout = timeout;
        let module_attach_api_client = LineModuleAttachApiClient::new(Arc::new(module_attach_conf));

        // shop
        let mut shop_conf = ShopConfiguration::with_client(client.clone());
        shop_conf.oauth_access_token = Some(token.to_owned());
        shop_conf.timeout = timeout;
        let shop_api_client = ShopApiClient::new(Arc::new(shop_conf));

        // webhook
        let mut webhook_conf = WebhookConfiguration::with_client(client.clone());
        webhook_conf.oauth_access_token = Some(token.to_owned());
        webhook_conf.timeout = timeout;
        let webhook_dummy_api_client = WebhookDummyApiClient::new(Arc::new(webhook_conf));

        LINE {
            channel_access_token_api_client,
            insight_api_client,
            liff_api_client,
            manage_audience_api_client,
            manage_audience_blob_api_client,
            messaging_api_client,
            messaging_api_blob_client,
            module_api_client,
            module_attach_api_client,
            shop_api_client,
            webhook_dummy_api_client,
        }
    }
}
