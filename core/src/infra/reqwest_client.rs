use crate::domain::{repository::HttpClient, request::Request, response::Response};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::str::FromStr;
use std::time::Instant;

pub struct ReqwestHttpClient {
    client: reqwest::Client,
}

impl ReqwestHttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn send(&self, request: &Request) -> Result<Response> {
        let method = reqwest::Method::from_str(&request.method)
            .with_context(|| format!("Invalid HTTP method: {}", request.method))?;

        let mut builder = self.client.request(method, &request.url);

        for (k, v) in &request.headers {
            builder = builder.header(k, v);
        }

        if let Some(body) = &request.body {
            builder = builder.body(body.clone());
        }

        let start = Instant::now();
        let resp = builder.send().await?;
        let latency = start.elapsed();

        let status = resp.status();
        let status_code = status.as_u16();
        let status_text = status.canonical_reason().unwrap_or("").to_string();
        
        let body_text = resp.text().await?;

        // Note: Headers are not copied yet for simplicity, can be added if needed by Response struct
        let response = Response::new(status_code, status_text, body_text, latency);

        Ok(response)
    }
}
