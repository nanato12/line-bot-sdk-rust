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

//! axum framework integration for LINE webhook signature extraction.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Extracts the `x-line-signature` header value from an axum request.
///
/// Implements [`FromRequestParts`] so it can be used as an extractor.
///
/// # Example
///
/// ```ignore
/// use axum::{routing::post, Router};
/// use line_bot_sdk_rust::support::axum::Signature;
///
/// async fn callback(signature: Signature, body: String) -> &'static str {
///     // Use signature.key for validation
///     "ok"
/// }
///
/// let app = Router::new().route("/callback", post(callback));
/// ```
#[derive(Debug)]
pub struct Signature {
    /// The value of the `x-line-signature` header.
    pub key: String,
}

/// Rejection type returned when the `x-line-signature` header is missing or invalid.
#[derive(Debug)]
pub struct SignatureRejection {
    message: &'static str,
}

impl IntoResponse for SignatureRejection {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.message).into_response()
    }
}

impl<S> FromRequestParts<S> for Signature
where
    S: Send + Sync,
{
    type Rejection = SignatureRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.headers.get("x-line-signature") {
            Some(value) => match value.to_str() {
                Ok(key) => Ok(Signature {
                    key: key.to_string(),
                }),
                Err(_) => Err(SignatureRejection {
                    message: "x-line-signature contains invalid characters",
                }),
            },
            None => Err(SignatureRejection {
                message: "x-line-signature is missing",
            }),
        }
    }
}
