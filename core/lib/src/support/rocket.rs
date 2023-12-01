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

use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

#[derive(Debug)]
pub struct Signature {
    pub key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Signature {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("x-line-signature");
        match token {
            Some(token) => Outcome::Success(Signature {
                key: token.to_string(),
            }),
            None => Outcome::Error((
                Status::BadRequest,
                "x-line-signature is missing".to_string(),
            )),
        }
    }
}
