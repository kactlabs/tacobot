//! Detailed error types for PicoClaw

use std::fmt;

/// Detailed error information for PicoClaw operations
#[derive(Debug, Clone)]
pub struct PicoClawError {
    pub code: ErrorCode,
    pub message: String,
    pub context: Option<String>,
}

/// Error codes for different error categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Configuration errors
    ConfigNotFound = 1001,
    ConfigInvalid = 1002,
    ConfigMissing = 1003,

    // Authentication errors
    AuthFailed = 2001,
    TokenExpired = 2002,
    TokenRefreshFailed = 2003,
    PkceInvalid = 2004,

    // Channel errors
    ChannelNotFound = 3001,
    ChannelConnectionFailed = 3002,
    ChannelMessageFailed = 3003,

    // LLM provider errors
    ProviderNotFound = 4001,
    ProviderUnavailable = 4002,
    ProviderRateLimited = 4003,
    ProviderInvalidResponse = 4004,

    // Tool errors
    ToolNotFound = 5001,
    ToolExecutionFailed = 5002,
    ToolTimeout = 5003,

    // Session errors
    SessionNotFound = 6001,
    SessionExpired = 6002,
    SessionPersistenceFailed = 6003,

    // Device errors
    DeviceNotFound = 7001,
    DeviceUnavailable = 7002,
    DeviceOperationFailed = 7003,

    // Internal errors
    InternalError = 9001,
    Unknown = 9999,
}

impl PicoClawError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        PicoClawError {
            code,
            message: message.into(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

impl fmt::Display for PicoClawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code as u32, self.message)?;
        if let Some(ctx) = &self.context {
            write!(f, " ({})", ctx)?;
        }
        Ok(())
    }
}
