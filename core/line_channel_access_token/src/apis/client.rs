/*
* Copyright (C) 2016 LINE Corp.
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

use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    channel_access_token_api: Box<dyn crate::apis::ChannelAccessTokenApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Rc::new(configuration);

        APIClient {
            channel_access_token_api: Box::new(crate::apis::ChannelAccessTokenApiClient::new(rc.clone())),
        }
    }

    pub fn channel_access_token_api(&self) -> &dyn crate::apis::ChannelAccessTokenApi{
        self.channel_access_token_api.as_ref()
    }

}
