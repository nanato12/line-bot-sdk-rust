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

use std::fmt;
use std::fmt::Debug;

use hyper;
use hyper::http;
use hyper_util::client::legacy::connect::Connect;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Header(http::header::InvalidHeaderValue),
    Http(http::Error),
    Hyper(hyper::Error),
    HyperClient(hyper_util::client::legacy::Error),
    Serde(serde_json::Error),
    UriError(http::uri::InvalidUri),
}

pub struct ApiError {
    pub code: hyper::StatusCode,
    pub body: hyper::body::Incoming,
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
            .field("code", &self.code)
            .field("body", &"hyper::body::Incoming")
            .finish()
    }
}

impl From<(hyper::StatusCode, hyper::body::Incoming)> for Error {
    fn from(e: (hyper::StatusCode, hyper::body::Incoming)) -> Self {
        Error::Api(ApiError {
            code: e.0,
            body: e.1,
        })
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error::Http(e)
    }
}

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(e: hyper_util::client::legacy::Error) -> Self {
        Error::HyperClient(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

mod request;

mod shop_api;
pub use self::shop_api::{ShopApi, ShopApiClient};

pub mod client;
pub mod configuration;
