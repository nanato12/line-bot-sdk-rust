//! Functions for webhook

use base64::encode;
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Signature validator
/// # Note
/// The signature in the `x-line-signature` request header must be verified to confirm that the request was sent from the LINE Platform. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#signature-validation)
/// # Example
/// ```
/// if webhook::validate_signature(channel_secret, signature, body) {
///     // OK
/// } else {
///     // NG
/// }
/// ```
pub fn validate_signature(channel_secret: &str, signature: &str, body: &str) -> bool {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac =
        HmacSha256::new_from_slice(channel_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(body.as_bytes());
    encode(&mac.finalize().into_bytes().to_vec()) == signature
}
