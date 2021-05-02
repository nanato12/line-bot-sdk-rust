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
    type Config = ();

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
