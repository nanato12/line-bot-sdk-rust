use std::rc::Rc;

use channel_access_token::apis::{
    configuration::Configuration as ChannelAccessTokenApiConfiguration, ChannelAccessTokenApiClient,
};
use hyper::{client::HttpConnector, Client as hyperClient};
use hyper_rustls::HttpsConnector;
use hyper_rustls::HttpsConnectorBuilder;
use insight::apis::{configuration::Configuration as InsightConfiguration, InsightApiClient};
use liff::apis::{configuration::Configuration as LiffConfiguration, LiffApiClient};
use manage_audience::apis::{
    configuration::Configuration as ManageAudienceConfiguration, ManageAudienceApiClient,
};
use messaging_api::apis::{
    configuration::Configuration as MessagingApiConfiguration, MessagingApiApiClient,
};
use module::apis::{configuration::Configuration as LineModuleConfiguration, LineModuleApiClient};
use module_attach::apis::{
    configuration::Configuration as LineModuleAttachConfiguration, LineModuleAttachApiClient,
};
use shop::apis::{configuration::Configuration as ShopConfiguration, ShopApiClient};
use webhook::apis::{
    configuration::Configuration as WebhookConfiguration, DummyApiClient as WebhookDummyApiClient,
};

type C = HttpsConnector<HttpConnector>;

pub struct LINE {
    pub channel_access_token_api_client: ChannelAccessTokenApiClient<C>,
    pub insight_api_client: InsightApiClient<C>,
    pub liff_api_client: LiffApiClient<C>,
    pub manage_audience_api_client: ManageAudienceApiClient<C>,
    pub messaging_api_client: MessagingApiApiClient<C>,
    pub module_api_client: LineModuleApiClient<C>,
    pub module_attach_api_client: LineModuleAttachApiClient<C>,
    pub shop_api_client: ShopApiClient<C>,
    pub webhook_dummy_api_client: WebhookDummyApiClient<C>,
}

impl LINE {
    pub fn new(token: String) -> LINE {
        let client = LINE::create_hyper_client();

        // channel_access_token_api
        let mut channel_access_token_api_conf =
            ChannelAccessTokenApiConfiguration::new(client.clone());
        channel_access_token_api_conf.oauth_access_token = Some(token.to_owned());
        let channel_access_token_api_client =
            ChannelAccessTokenApiClient::new(Rc::new(channel_access_token_api_conf));

        // insight
        let mut insight_conf = InsightConfiguration::new(client.clone());
        insight_conf.oauth_access_token = Some(token.to_owned());
        let insight_api_client = InsightApiClient::new(Rc::new(insight_conf));

        // liff
        let mut liff_conf = LiffConfiguration::new(client.clone());
        liff_conf.oauth_access_token = Some(token.to_owned());
        let liff_api_client = LiffApiClient::new(Rc::new(liff_conf));

        // manage_audience
        let mut manage_audience_conf = ManageAudienceConfiguration::new(client.clone());
        manage_audience_conf.oauth_access_token = Some(token.to_owned());
        let manage_audience_api_client =
            ManageAudienceApiClient::new(Rc::new(manage_audience_conf));

        // messaging_api
        let mut messaging_api_conf = MessagingApiConfiguration::new(client.clone());
        messaging_api_conf.oauth_access_token = Some(token.to_owned());
        let messaging_api_client = MessagingApiApiClient::new(Rc::new(messaging_api_conf));

        // module
        let mut module_conf = LineModuleConfiguration::new(client.clone());
        module_conf.oauth_access_token = Some(token.to_owned());
        let module_api_client = LineModuleApiClient::new(Rc::new(module_conf));

        // module_attach
        let mut module_attach_conf = LineModuleAttachConfiguration::new(client.clone());
        module_attach_conf.oauth_access_token = Some(token.to_owned());
        let module_attach_api_client = LineModuleAttachApiClient::new(Rc::new(module_attach_conf));

        // shop
        let mut shop_conf = ShopConfiguration::new(client.clone());
        shop_conf.oauth_access_token = Some(token.to_owned());
        let shop_api_client = ShopApiClient::new(Rc::new(shop_conf));

        // webhook
        let mut webhook_conf = WebhookConfiguration::new(client.clone());
        webhook_conf.oauth_access_token = Some(token.to_owned());
        let webhook_dummy_api_client = WebhookDummyApiClient::new(Rc::new(webhook_conf));

        LINE {
            channel_access_token_api_client,
            insight_api_client,
            liff_api_client,
            manage_audience_api_client,
            messaging_api_client,
            module_api_client,
            module_attach_api_client,
            shop_api_client,
            webhook_dummy_api_client,
        }
    }

    fn create_hyper_client() -> hyperClient<C> {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        hyperClient::builder().build::<_, hyper::Body>(https)
    }
}
