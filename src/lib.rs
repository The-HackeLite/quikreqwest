use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue}, StatusCode
};
use serde_json::Value;

#[derive(Debug)]
pub enum RequestError {
    BadRequest(StatusCode),
    NotFound,
    FailedRequest(reqwest::Error),
    Timeout,
    RateLimit,
    SerializationError
}

#[derive(Clone)]
pub struct Request {
    client: reqwest::Client,
}

impl Request {
    pub fn new(user_agent: &str, headers: &[(&'static str, &'static str)]) -> Self {
        let default_headers = Self::headers(headers);
        let client_builder = reqwest::Client::builder()
            .user_agent(user_agent)
            .default_headers(default_headers)
            .build();
        if let Ok(client) = client_builder {
            Request {
                client
            }
        } else {
            panic!("Failed to create a Client");
        }
    }

    async fn result_text(response: Result<reqwest::Response, reqwest::Error>) -> Result<String, RequestError> {
        match response {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK | StatusCode::FOUND | StatusCode::ACCEPTED => {
                        let text = response.text().await;
                        if let Ok(text) = text {
                            return Ok(text);
                        } else {
                            return Err(RequestError::SerializationError)
                        }
                    }

                    StatusCode::NOT_FOUND => {
                        return Err(RequestError::NotFound);
                    }

                    _code => {
                        return Err(RequestError::BadRequest(_code));
                    }
                }
            }
            Err(error) => {
                return Err(RequestError::FailedRequest(error));
            }
        }
    }

    async fn result_json(response: Result<reqwest::Response, reqwest::Error>) -> Result<serde_json::Value, RequestError> {
        match response {
            Ok(response) => {
                match response.status() {
                    StatusCode::FORBIDDEN => {
                        return Err(RequestError::BadRequest(StatusCode::FORBIDDEN));
                    }

                    _ => {
                        let value = response.json().await;
                        if let Ok(value) = value {
                            return Ok(value);
                        } else {
                            return Err(RequestError::SerializationError)
                        }
                    }
                }
            }
            Err(error) => {
                return Err(RequestError::FailedRequest(error));
            }
        }
    }

    pub fn headers(headers: &[(&'static str, &'static str)]) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        for (h_key, h_value) in headers {
            let header_key: &'static str = h_key.trim();
            let header_value: &'static str = h_value.trim();
            header_map.insert(HeaderName::from_static(header_key), HeaderValue::from_static(header_value));
        }
        header_map
    }

    pub async fn get_text(&self, url: &str) -> Result<String, RequestError> {
        let response = self.client.get(url).send().await;
        let text = Self::result_text(response).await;
        text
    }

    pub async fn get_text_with_headers(
        &self,
        url: &str,
        headers: HeaderMap,
    ) -> Result<String, RequestError> {
        let response = self.client.get(url).headers(headers).send().await;
        let text = Self::result_text(response).await;
        text
    }

    pub async fn get_json(
        &self,
        url: &str,
    ) -> Result<Value, RequestError> {
        let response = self.client.get(url).send().await;
        let json = Self::result_json(response).await;
        json
    }

    pub async fn get_json_with_headers(
        &self,
        url: &str,
        headers: HeaderMap,
    ) -> Result<Value, RequestError> {
        let response = self.client.get(url).headers(headers).send().await;
        let json = Self::result_json(response).await;
        json
    }
}