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

use super::configuration::Configuration;
use hyper;

pub struct APIClient {
    manage_audience_api: Box<dyn crate::apis::ManageAudienceApi>,
    manage_audience_blob_api: Box<dyn crate::apis::ManageAudienceBlobApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
    where
        C: Clone + std::marker::Send + Sync + 'static,
    {
        let rc = Rc::new(configuration);

        APIClient {
            manage_audience_api: Box::new(crate::apis::ManageAudienceApiClient::new(rc.clone())),
            manage_audience_blob_api: Box::new(crate::apis::ManageAudienceBlobApiClient::new(
                rc.clone(),
            )),
        }
    }

    pub fn manage_audience_api(&self) -> &dyn crate::apis::ManageAudienceApi {
        self.manage_audience_api.as_ref()
    }

    pub fn manage_audience_blob_api(&self) -> &dyn crate::apis::ManageAudienceBlobApi {
        self.manage_audience_blob_api.as_ref()
    }
}
