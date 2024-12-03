use std::collections::HashMap;

use bt_logger::{log_error, log_trace};
use reqwest::{header::{self, HeaderMap}, Client, Response, StatusCode};

pub(crate) struct HttpClient {
    client: Client,
}


#[derive(Clone, Debug)]
pub(crate) struct HttpResponse {
    pub status_code: u16,
    pub header: HashMap<String, String>,
    pub body: String,
}

pub(crate) enum ContentType{
    JSON,
    //TEXT,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> HttpResponse {
        let resp = self
            .client
            .get( url)
            .send()
            .await
            .expect(format!("Failed to get response from GET: {}",url).as_str());

            Self::extract_response(resp, url, "GET").await
    }

    pub async fn post(&self, url: &str, body_request: &str, content_type: ContentType) -> HttpResponse{
        let resp: Response;
        log_trace!("post","Getting {} with payload: {}", url, body_request);
        match content_type{
            ContentType::JSON => {resp = self.client
                                        .post(url)
                                        .header(header::CONTENT_TYPE, "application/json")
                                        .body(body_request.to_owned())
                                        .send()
                                        .await
                                        .expect(format!("Failed to get response from POST: {}",url).as_str())},
            /*ContentType::TEXT => {resp = self.client
                                        .post(url)
                                        .body(body_request.to_string())
                                        .send()
                                        .await
                                        .expect(format!("Failed to get response from POST: {}",url).as_str())},*/
        }

        Self::extract_response(resp, url, "POST").await
    }

    async fn extract_response(resp: Response, url: &str, method: &str) -> HttpResponse{
        if resp.status().is_client_error() || resp.status().is_server_error() {
            log_error!("extract_response", "ERROR: Failed to get response from {}: {} Error: {}", method, url, resp.status().canonical_reason().unwrap_or(""));
            return HttpResponse{
                status_code: resp.status().as_u16(),
                header: Self::convert_headers(resp.headers()),
                body: format!("ERROR: Failed to get response from {}: {} Error: {}",method, url, resp.status().canonical_reason().unwrap_or("")),
            }
        }else{
            return HttpResponse{
                status_code: resp.status().as_u16(),
                header:Self::convert_headers(resp.headers()),
                body: resp
                        .text()
                        .await
                        .expect(format!("ERROR: Failed to get payload from {}",method).as_str()),
            }
        }
    }

    fn convert_headers(headers: &HeaderMap) -> HashMap<String, String> {
        headers.iter().map(|(k, v)| {
            (
                k.to_string(),
                v.to_str().unwrap_or_default().to_string(),
            )
        }).collect()
    }
}

impl HttpResponse {
    pub fn is_error(&self) -> bool{
        let sc = StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::FORBIDDEN);
        sc.is_client_error() || sc.is_server_error()
    }
}
