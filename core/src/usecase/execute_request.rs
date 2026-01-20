use crate::domain::{
    repository::HttpClient,
    request::Request,
    response::Response,
};
use anyhow::Result;
use std::sync::Arc;

pub struct ExecuteRequestUseCase {
    client: Arc<dyn HttpClient>,
}

impl ExecuteRequestUseCase {
    pub fn new(client: Arc<dyn HttpClient>) -> Self {
        Self { client }
    }

    pub async fn execute(&self, request: &Request) -> Result<Response> {
        // Here we could add logic to save history, substitute env vars, etc.
        // For now, just pass through.
        self.client.send(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::MockHttpClient;
    use std::time::Duration;

    #[tokio::test]
    async fn test_execute_request() {
        let mut mock_client = MockHttpClient::new();
        let request = Request::new("GET", "https://example.com");
        let expected_request = request.clone();

        mock_client
            .expect_send()
            .withf(move |req| req.method == expected_request.method && req.url == expected_request.url)
            .times(1)
            .returning(|_| Ok(Response::new(200, "OK".to_string(), "body".to_string(), Duration::from_millis(100))));

        let usecase = ExecuteRequestUseCase::new(Arc::new(mock_client));
        let response = usecase.execute(&request).await.unwrap();

        assert_eq!(response.status, 200);
        assert_eq!(response.body, "body");
    }
}
