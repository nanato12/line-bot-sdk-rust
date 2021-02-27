//! HTTP Client

use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Error;
use reqwest::Url;
use serde_json::Value;

static BASE_URL: &str = "https://api.line.me/v2/bot";

#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    headers: HeaderMap,
    endpoint_base: String,
}

impl HttpClient {
    /// # Note
    /// Instantiate a HttpClient.
    /// ```
    /// let http_client = HttpClient::new("<channel secret>");
    /// ```
    pub fn new(channel_token: &str) -> HttpClient {
        let mut headers = HeaderMap::new();
        if let Ok(v) = format!("Bearer {}", channel_token).parse::<String>() {
            if let Ok(header_value) = HeaderValue::from_str(&v) {
                headers.insert(AUTHORIZATION, header_value);
            }
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        HttpClient {
            client: Client::new(),
            headers: headers,
            endpoint_base: String::from(BASE_URL),
        }
    }

    /// # Note
    /// `GET` request
    /// ```
    /// let res: Result<Response, Error> = http_client.get("https://example.com");
    /// ```
    pub fn get(
        &self,
        endpoint: &str,
        query: Vec<(&str, &str)>,
        data: Value,
    ) -> Result<Response, Error> {
        let uri = Url::parse(&format!("{}{}", self.endpoint_base, endpoint)).unwrap();
        self.client
            .get(uri)
            .query(&query)
            .headers(self.headers.clone())
            .json(&data)
            .send()
    }

    /// # Note
    /// `POST` request
    /// ```
    /// let res: Result<Response, Error> = http_client.post("https://example.com");
    /// ```
    pub fn post(&self, endpoint: &str, data: Value) -> Result<Response, Error> {
        let uri = Url::parse(&format!("{}{}", self.endpoint_base, endpoint)).unwrap();
        self.client
            .post(uri)
            .headers(self.headers.clone())
            .json(&data)
            .send()
    }

    /// # Note
    /// `PUT` request
    /// ```
    /// let res: Result<Response, Error> = http_client.put("https://example.com");
    /// ```
    pub fn put(&self, endpoint: &str, data: Value) -> Result<Response, Error> {
        let uri = Url::parse(&format!("{}{}", self.endpoint_base, endpoint)).unwrap();
        self.client
            .put(uri)
            .headers(self.headers.clone())
            .json(&data)
            .send()
    }

    /// # Note
    /// `DELETE` request
    /// ```
    /// let res: Result<Response, Error> = http_client.delete("https://example.com");
    /// ```
    pub fn delete(&self, endpoint: &str, data: Value) -> Result<Response, Error> {
        let uri = Url::parse(&format!("{}{}", self.endpoint_base, endpoint)).unwrap();
        self.client
            .delete(uri)
            .headers(self.headers.clone())
            .json(&data)
            .send()
    }
}
