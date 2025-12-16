use anyhow::{Context, Result};
use reqwest::{Client, Method, Response, StatusCode};
use serde_json::Value;
use std::time::Duration;

#[derive(Clone)]
pub struct GogsClient {
    base_url: String,
    token: String,
    client: Client,
}

impl GogsClient {
    pub fn new(base_url: String, token: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        // Remove trailing slash from base_url if present
        let base_url = base_url.trim_end_matches('/').to_string();

        Self {
            base_url,
            token,
            client,
        }
    }

    pub async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<Response> {
        let url = format!("{}/api/v1{}", self.base_url, path);

        let mut req = self
            .client
            .request(method, &url)
            .header("Authorization", format!("token {}", self.token))
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            req = req.json(&body);
        }

        let resp = req.send().await.context("Failed to send request")?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();

            if status == StatusCode::UNAUTHORIZED {
                anyhow::bail!("Authentication failed. Check your API token.");
            } else if status == StatusCode::NOT_FOUND {
                anyhow::bail!("Resource not found: {}", text);
            } else if status == StatusCode::FORBIDDEN {
                anyhow::bail!("Access denied. Check permissions for this resource.");
            } else {
                anyhow::bail!("API error {}: {}", status, text);
            }
        }

        Ok(resp)
    }

    pub async fn get(&self, path: &str) -> Result<Response> {
        self.request(Method::GET, path, None).await
    }

    pub async fn post(&self, path: &str, body: Value) -> Result<Response> {
        self.request(Method::POST, path, Some(body)).await
    }

    pub async fn patch(&self, path: &str, body: Value) -> Result<Response> {
        self.request(Method::PATCH, path, Some(body)).await
    }

    #[allow(dead_code)]
    pub async fn delete(&self, path: &str) -> Result<Response> {
        self.request(Method::DELETE, path, None).await
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
