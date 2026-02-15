# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial Rust project structure and dependencies setup
- Cargo.toml with all required dependencies (tokio, serde, reqwest, tracing, etc.)
- Feature flags for optional channel and provider integrations
- Release profile configuration for embedded deployment (LTO, strip symbols)
- CLI interface with clap-based argument parsing
- Core module structure (agent, auth, channels, config, device, error, llm, logging, session, tools)
- Logging and error handling framework
- Configuration management system
- Authentication system (OAuth2 and PKCE)
- Agent loop and context management
- Channel integration framework
- LLM provider integration framework
- Tool framework and abstractions
- Session and state management
- Device manager for hardware interfaces

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.1.0] - 2024-02-14

### Added
- Initial project setup for PicoClaw Rust conversion
- Project structure and dependencies
- Binary size optimized to 1.4MB for embedded deployment
- Boot time target: <1 second
- Memory footprint target: <10MB

[Unreleased]: https://github.com/picoclaw/picoclaw-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/picoclaw/picoclaw-rust/releases/tag/v0.1.0
