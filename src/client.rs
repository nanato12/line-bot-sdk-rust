static BASE_URL: &str = "https://api.line.me/v2/bot";

#[derive(Debug)]
pub struct HttpClient {
    endpoint_base: String,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {
            endpoint_base: String::from(BASE_URL),
        }
    }

    // TODO: get request
    // TODO: post request
    // TODO: put request
    // TODO: delete request
}
