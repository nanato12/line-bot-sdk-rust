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
    line_module_attach: super::LineModuleAttachApiClient<C>,
}

impl<C: Connect> APIClient<C>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Arc::new(configuration);

        APIClient {
            line_module_attach: super::LineModuleAttachApiClient::new(rc.clone()),
        }
    }
    pub fn line_module_attach(&self) -> &super::LineModuleAttachApiClient<C> {
        &self.line_module_attach
    }
}
