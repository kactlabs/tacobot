//! Simple LLM client for making requests to various providers

use serde_json::json;
use crate::error::{Error, Result};

pub struct LlmClient {
    provider: String,
    model: String,
    api_key: String,
    api_base: String,
}

impl LlmClient {
    pub fn new(provider: &str, model: &str, api_key: &str, api_base: &str) -> Self {
        Self {
            provider: provider.to_string(),
            model: model.to_string(),
            api_key: api_key.to_string(),
            api_base: api_base.to_string(),
        }
    }

    pub async fn chat(&self, message: &str) -> Result<String> {
        match self.provider.as_str() {
            "openrouter" => self.chat_openrouter(message).await,
            "openai" => self.chat_openai(message).await,
            "anthropic" => self.chat_anthropic(message).await,
            _ => Err(Error::llm_provider(format!(
                "Unsupported provider: {}",
                self.provider
            ))),
        }
    }

    async fn chat_openrouter(&self, message: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.api_base);

        let payload = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "user",
                    "content": message
                }
            ],
            "temperature": 0.7,
            "max_tokens": 2048,
        });

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::http(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::llm_provider(format!(
                "API error {}: {}",
                status, text
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::serialization(format!("Failed to parse response: {}", e)))?;

        data["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::llm_provider("No content in response".to_string()))
    }

    async fn chat_openai(&self, message: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.api_base);

        let payload = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "user",
                    "content": message
                }
            ],
            "temperature": 0.7,
            "max_tokens": 2048,
        });

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::http(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::llm_provider(format!(
                "API error {}: {}",
                status, text
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::serialization(format!("Failed to parse response: {}", e)))?;

        data["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::llm_provider("No content in response".to_string()))
    }

    async fn chat_anthropic(&self, message: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let url = format!("{}/messages", self.api_base);

        let payload = json!({
            "model": self.model,
            "max_tokens": 2048,
            "messages": [
                {
                    "role": "user",
                    "content": message
                }
            ],
        });

        let response = client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::http(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::llm_provider(format!(
                "API error {}: {}",
                status, text
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::serialization(format!("Failed to parse response: {}", e)))?;

        data["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::llm_provider("No content in response".to_string()))
    }
}
