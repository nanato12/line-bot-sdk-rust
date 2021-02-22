use base64::encode;
use hmac::{Hmac, Mac};
use sha2::Sha256;

// Signature confirmation
pub fn validate_signature(channel_secret: &str, signature: &str, body: &str) -> bool {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac =
        HmacSha256::new_varkey(channel_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.input(body.as_bytes());
    encode(&mac.result().code().to_vec()) == signature
}
