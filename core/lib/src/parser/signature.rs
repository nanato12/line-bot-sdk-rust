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

/// Validate the signature of a webhook request using HMAC-SHA256.
///
/// The signature in the `x-line-signature` request header must be verified
/// to confirm that the request was sent from the LINE Platform.
/// [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#signature-validation)
///
/// This function uses constant-time comparison to prevent timing side-channel attacks.
///
/// # Example
/// ```
/// use line_bot_sdk_rust::parser::signature::validate_signature;
///
/// let is_valid = validate_signature("channel_secret", "signature", "body");
/// ```
pub fn validate_signature(channel_secret: &str, signature: &str, body: &str) -> bool {
    type HmacSha256 = Hmac<Sha256>;

    let Ok(mut mac) = HmacSha256::new_from_slice(channel_secret.as_bytes()) else {
        return false;
    };
    mac.update(body.as_bytes());

    let Ok(decoded_signature) = general_purpose::STANDARD.decode(signature) else {
        return false;
    };

    // verify_slice uses constant-time comparison internally
    mac.verify_slice(&decoded_signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_signature() {
        let channel_secret = "test_secret";
        let body = r#"{"events":[]}"#;

        // Generate expected signature
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes()).unwrap();
        mac.update(body.as_bytes());
        let expected = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        assert!(validate_signature(channel_secret, &expected, body));
    }

    #[test]
    fn test_invalid_signature() {
        assert!(!validate_signature(
            "test_secret",
            "aW52YWxpZA==",
            r#"{"events":[]}"#
        ));
    }

    #[test]
    fn test_malformed_base64_signature() {
        assert!(!validate_signature(
            "test_secret",
            "not-valid-base64!!!",
            "body"
        ));
    }

    #[test]
    fn test_empty_body() {
        let channel_secret = "secret";
        let body = "";

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes()).unwrap();
        mac.update(body.as_bytes());
        let expected = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        assert!(validate_signature(channel_secret, &expected, body));
    }

    #[test]
    fn test_empty_signature() {
        assert!(!validate_signature("secret", "", "body"));
    }

    #[test]
    fn test_different_secret_produces_different_result() {
        let body = r#"{"events":[]}"#;

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(b"secret_a").unwrap();
        mac.update(body.as_bytes());
        let sig_a = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        assert!(!validate_signature("secret_b", &sig_a, body));
    }
}
