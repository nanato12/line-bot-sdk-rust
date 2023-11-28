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

use actix_web::dev::Payload;
use actix_web::{error::ErrorBadRequest, Error, FromRequest, HttpRequest};
use std::{future::Future, pin::Pin};

#[derive(Debug)]
pub struct Signature {
    pub key: String,
}

impl FromRequest for Signature {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let res = if let Some(x_line_signature) = req.headers().get("x-line-signature") {
            if let Ok(key) = x_line_signature.to_str() {
                Ok(Signature {
                    key: key.to_string(),
                })
            } else {
                Err(ErrorBadRequest("x-line-signature is missing"))
            }
        } else {
            Err(ErrorBadRequest("x-line-signature is missing"))
        };
        Box::pin(async move { res })
    }
}
