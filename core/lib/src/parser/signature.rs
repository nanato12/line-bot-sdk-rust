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

//! Functions for parser

use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Signature validator
/// # Note
/// The signature in the `x-line-signature` request header must be verified to confirm that the request was sent from the LINE Platform. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#signature-validation)
/// # Example
/// ```
/// if validate_signature(channel_secret, signature, body) {
///     // OK
/// } else {
///     // NG
/// }
/// ```
pub fn validate_signature(channel_secret: String, signature: String, body: String) -> bool {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(body.as_bytes());
    let mut buf = String::new();
    general_purpose::STANDARD.encode_string(&mac.finalize().into_bytes().to_vec(), &mut buf);
    buf == signature
}
