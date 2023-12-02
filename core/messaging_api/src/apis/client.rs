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

use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient {
    messaging_api_api: Box<dyn crate::apis::MessagingApiApi>,
    messaging_api_blob_api: Box<dyn crate::apis::MessagingApiBlobApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
    where
        C: Clone + std::marker::Send + Sync + 'static,
    {
        let rc = Rc::new(configuration);

        APIClient {
            messaging_api_api: Box::new(crate::apis::MessagingApiApiClient::new(rc.clone())),
            messaging_api_blob_api: Box::new(crate::apis::MessagingApiBlobApiClient::new(
                rc.clone(),
            )),
        }
    }

    pub fn messaging_api_api(&self) -> &dyn crate::apis::MessagingApiApi {
        self.messaging_api_api.as_ref()
    }

    pub fn messaging_api_blob_api(&self) -> &dyn crate::apis::MessagingApiBlobApi {
        self.messaging_api_blob_api.as_ref()
    }
}
