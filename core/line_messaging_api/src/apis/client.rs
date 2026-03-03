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

use std::sync::Arc;

use super::configuration::Configuration;
use hyper;
use hyper_util::client::legacy::connect::Connect;

pub struct APIClient<C: Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    messaging_api: crate::apis::MessagingApiApiClient<C>,
    messaging_api_blob: crate::apis::MessagingApiBlobApiClient<C>,
}

impl<C: Connect> APIClient<C>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Arc::new(configuration);

        APIClient {
            messaging_api: crate::apis::MessagingApiApiClient::new(rc.clone()),
            messaging_api_blob: crate::apis::MessagingApiBlobApiClient::new(rc.clone()),
        }
    }
    pub fn messaging_api(&self) -> &crate::apis::MessagingApiApiClient<C> {
        &self.messaging_api
    }
    pub fn messaging_api_blob(&self) -> &crate::apis::MessagingApiBlobApiClient<C> {
        &self.messaging_api_blob
    }
}
