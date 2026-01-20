use std::collections::HashMap;
use std::time::Duration;

/// Represents an HTTP Response in the domain.
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub latency: Duration,
}

impl Response {
    pub fn new(status: u16, status_text: String, body: String, latency: Duration) -> Self {
        Self {
            status,
            status_text,
            headers: HashMap::new(),
            body,
            latency,
        }
    }
}
