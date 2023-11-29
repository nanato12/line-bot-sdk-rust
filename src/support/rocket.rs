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
    data::FromData,
    http::{ContentType, Status},
    request::{FromRequest, Outcome, Request},
    Data,
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

/// HTTP Request body
#[derive(Debug)]
pub struct Body {
    pub string: String,
}

// #[rocket::async_trait]
// impl<'r> FromData<'r> for Body {
//     type Error = String;

//     async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<Self, Self::Error> {
//         // ContentType must be application/json
//         let body_ct = ContentType::new("application", "json");
//         if req.content_type() != Some(&body_ct) {
//             return Outcome::Forward(data);
//         }
//         let limit = req.limits().get("person").unwrap_or(256_i32.bytes());

//         // Parse data to String
//         let mut string = data.open(limit).into_string().await.unwrap();
//         if !string.is_complete() {
//             return Outcome::Error((Status::InternalServerError, "ng".to_string()));
//         }
//         // Return successfully.
//         Outcome::Success(Body {
//             string: string.into_inner(),
//         })
//     }
// }
