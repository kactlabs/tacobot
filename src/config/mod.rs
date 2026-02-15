//! Configuration management for PicoClaw

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub agent: AgentConfig,
    pub channels: ChannelsConfig,
    pub llm: LlmConfig,
    pub tools: ToolsConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub max_context_size: usize,
    pub timeout_ms: u64,
    pub memory_limit_mb: usize,
}

/// Channels configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsConfig {
    pub telegram: Option<ChannelConfig>,
    pub discord: Option<ChannelConfig>,
}

/// Individual channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub enabled: bool,
    pub token: Option<String>,
}

/// LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub default_provider: String,
    pub providers: HashMap<String, ProviderConfig>,
}

/// LLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    pub model: Option<String>,
}

/// Tools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub web_search: Option<ToolConfig>,
    pub filesystem: Option<ToolConfig>,
    pub shell: Option<ToolConfig>,
}

/// Individual tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub enabled: bool,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub oauth_enabled: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            agent: AgentConfig {
                max_context_size: 8192,
                timeout_ms: 5000,
                memory_limit_mb: 10,
            },
            channels: ChannelsConfig {
                telegram: None,
                discord: None,
            },
            llm: LlmConfig {
                default_provider: "openrouter".to_string(),
                providers: HashMap::new(),
            },
            tools: ToolsConfig {
                web_search: None,
                filesystem: None,
                shell: None,
            },
            auth: AuthConfig {
                oauth_enabled: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        }
    }
}
