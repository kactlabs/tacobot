# Implementation Plan: PicoClaw Go-to-Rust Conversion

## Overview

This implementation plan breaks down the PicoClaw Go-to-Rust conversion into discrete, testable coding tasks organized across 6 phases. Each phase delivers a complete, independently functional subsystem. Tasks are ordered to build incrementally, with each task building on previous work. Property-based tests are integrated throughout to validate correctness properties early.

## Phase 1: Core Infrastructure (Weeks 1-2)

- [ ] 1. Set up Rust project structure and dependencies
  - Create Cargo.toml with all required dependencies (tokio, serde, reqwest, tracing, etc.)
  - Set up feature flags for optional channel and provider integrations
  - Configure Cargo.toml for embedded deployment (LTO, strip symbols)
  - Create src/lib.rs and src/main.rs entry points
  - _Requirements: 2.1, 30.1, 30.2, 30.3, 30.4_

- [ ] 2. Implement async runtime initialization
  - Create src/runtime/mod.rs with tokio runtime setup
  - Implement graceful shutdown mechanism for all async tasks
  - Create task pool for managing concurrent operations
  - _Requirements: 2.1, 2.2, 2.4, 2.5_

  - [ ]* 2.1 Write property test for runtime initialization
    - **Property 2: Runtime initialization within 100ms**
    - **Validates: Requirements 2.2**

  - [ ]* 2.2 Write property test for async error propagation
    - **Property 3: Error handling for async failures**
    - **Validates: Requirements 2.3**

  - [ ]* 2.3 Write property test for graceful shutdown
    - **Property 4: Graceful shutdown completeness**
    - **Validates: Requirements 2.4**

- [ ] 3. Implement configuration management system
  - Create src/config/mod.rs with Config struct
  - Implement YAML/TOML file loading in src/config/loader.rs
  - Implement environment variable override logic
  - Create configuration validation in src/config/schema.rs
  - _Requirements: 3.1, 3.2, 3.4, 3.5_

  - [ ]* 3.1 Write property test for config round-trip
    - **Property 1: Configuration round-trip consistency**
    - **Validates: Requirements 3.1, 3.5**

  - [ ]* 3.2 Write property test for env var overrides
    - **Property 2: Environment variable override**
    - **Validates: Requirements 3.2**

  - [ ]* 3.3 Write unit test for invalid config handling
    - Test specific invalid configs return descriptive errors
    - _Requirements: 3.3_

- [ ] 4. Implement logging and error handling framework
  - Create src/error/types.rs with comprehensive error types
  - Create src/logging/setup.rs with tracing initialization
  - Implement structured logging with multiple output targets
  - Create error context capture for debugging
  - _Requirements: 25.1, 25.2, 25.3, 25.4_

  - [ ]* 4.1 Write unit test for error context capture
    - Test that error context is captured correctly
    - _Requirements: 25.2_

  - [ ]* 4.2 Write property test for log level filtering
    - **Property 25: Log level filtering**
    - **Validates: Requirements 25.3**

- [ ] 5. Implement CLI interface and argument parsing
  - Create src/main.rs with clap-based CLI argument parsing
  - Implement same command-line interface as Go version
  - Implement help text and usage information
  - Implement exit code handling matching Go version
  - _Requirements: 26.1, 26.2, 26.4_

  - [ ]* 5.1 Write property test for CLI argument compatibility
    - **Property 29: CLI argument compatibility**
    - **Validates: Requirements 26.1**

  - [ ]* 5.2 Write property test for exit code consistency
    - **Property 30: Exit code consistency**
    - **Validates: Requirements 26.4**

- [ ] 6. Checkpoint - Verify core infrastructure
  - Ensure all tests pass
  - Verify binary compiles and runs
  - Check memory usage is under 10MB
  - Ask the user if questions arise

## Phase 2: Authentication and Agent Loop (Weeks 3-4)

- [ ] 7. Implement OAuth2 and PKCE authentication
  - Create src/auth/mod.rs with OAuth2 types
  - Implement PKCE challenge generation in src/auth/pkce.rs
  - Implement OAuth2 flow in src/auth/oauth2.rs
  - Implement token storage in src/auth/token_storage.rs
  - _Requirements: 4.1, 4.2, 4.3, 4.4_

  - [ ]* 7.1 Write property test for PKCE challenge validity
    - **Property 3: PKCE challenge validity**
    - **Validates: Requirements 4.1, 4.2**

  - [ ]* 7.2 Write property test for token persistence
    - **Property 4: Token persistence round-trip**
    - **Validates: Requirements 4.3**

  - [ ]* 7.3 Write property test for concurrent sessions
    - **Property 5: Concurrent session isolation**
    - **Validates: Requirements 4.5**

- [ ] 8. Implement session and state management
  - Create src/session/manager.rs with SessionManager
  - Implement session persistence in src/session/store.rs
  - Implement session metadata handling
  - Implement session expiry and cleanup
  - _Requirements: 23.1, 23.2, 23.3, 23.4, 23.5_

  - [ ]* 8.1 Write property test for session uniqueness
    - **Property 21: Session uniqueness**
    - **Validates: Requirements 23.1**

  - [ ]* 8.2 Write property test for session persistence
    - **Property 20: Session persistence round-trip**
    - **Validates: Requirements 23.2, 23.3**

- [ ] 9. Implement memory management subsystem
  - Create src/agent/memory.rs with MemoryManager
  - Implement conversation history with size limits
  - Implement eviction policies (time-based, size-based)
  - Implement memory usage monitoring
  - _Requirements: 22.1, 22.2, 22.3, 22.4_

  - [ ]* 9.1 Write property test for memory limit enforcement
    - **Property 19: Memory usage constraint**
    - **Validates: Requirements 22.1, 27.2**

  - [ ]* 9.2 Write property test for history eviction
    - **Property 8: History size enforcement**
    - **Validates: Requirements 22.2**

- [ ] 10. Implement device manager
  - Create src/device/manager.rs with DeviceManager
  - Implement device discovery in src/device/mod.rs
  - Implement I2C interface in src/device/i2c.rs
  - Implement SPI interface in src/device/spi.rs
  - _Requirements: 24.1, 24.2, 24.3, 24.4_

  - [ ]* 10.1 Write property test for device registry
    - **Property 22: Device registry consistency**
    - **Validates: Requirements 24.2, 24.3**

- [ ] 11. Implement agent loop and context management
  - Create src/agent/loop.rs with AgentLoop
  - Implement context creation in src/agent/context.rs
  - Implement message processing pipeline
  - Implement tool execution orchestration
  - Implement error recovery and timeout handling
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 5.6_

  - [ ]* 11.1 Write property test for message processing order
    - **Property 6: Message processing order**
    - **Validates: Requirements 5.1**

  - [ ]* 11.2 Write property test for context completeness
    - **Property 7: Context completeness**
    - **Validates: Requirements 5.2**

  - [ ]* 11.3 Write property test for error isolation
    - **Property 27: Error isolation**
    - **Validates: Requirements 5.5, 28.3**

- [ ] 12. Checkpoint - Verify authentication and agent loop
  - Ensure all tests pass
  - Verify agent loop processes messages correctly
  - Verify session persistence works
  - Ask the user if questions arise

## Phase 3: Channel Integrations (Weeks 5-7)

- [ ] 13. Implement channel framework and abstractions
  - Create src/channels/framework.rs with Channel trait
  - Implement ChannelManager in src/channels/mod.rs
  - Implement message normalization
  - Implement reconnection logic with exponential backoff
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5, 6.6_

  - [ ]* 13.1 Write property test for message normalization
    - **Property 9: Channel message normalization**
    - **Validates: Requirements 6.2**

  - [ ]* 13.2 Write property test for concurrent channel operations
    - **Property 10: Concurrent channel operations**
    - **Validates: Requirements 6.3**

  - [ ]* 13.3 Write property test for response routing
    - **Property 11: Response routing correctness**
    - **Validates: Requirements 6.4**

  - [ ]* 13.4 Write property test for reconnection
    - **Property 26: Channel reconnection success**
    - **Validates: Requirements 6.6, 28.1**

- [ ] 14. Implement Telegram channel integration
  - Create src/channels/telegram.rs with TelegramChannel
  - Implement message receiving (polling and webhook modes)
  - Implement message sending with formatting
  - Implement Telegram-specific features (keyboards, file uploads)
  - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

  - [ ]* 14.1 Write unit test for Telegram message handling
    - Test message receiving and sending
    - _Requirements: 7.1, 7.3_

- [ ] 15. Implement Discord channel integration
  - Create src/channels/discord.rs with DiscordChannel
  - Implement websocket connection to Discord
  - Implement message receiving and sending
  - Implement Discord-specific features (embeds, reactions, threads)
  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

  - [ ]* 15.1 Write unit test for Discord message handling
    - Test message receiving and sending
    - _Requirements: 8.1, 8.3_

- [ ] 16. Implement additional channel integrations
  - Create src/channels/dingtalk.rs with DingTalkChannel
  - Create src/channels/line.rs with LineChannel
  - Create src/channels/qq.rs with QQChannel
  - Create src/channels/whatsapp.rs with WhatsAppChannel
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5_

  - [ ]* 16.1 Write unit tests for additional channels
    - Test each channel's message handling
    - _Requirements: 9.1, 9.2, 9.3, 9.4_

- [ ] 17. Checkpoint - Verify channel integrations
  - Ensure all tests pass
  - Verify all channels can send and receive messages
  - Ask the user if questions arise

## Phase 4: LLM Providers (Weeks 8-10)

- [ ] 18. Implement LLM provider framework
  - Create src/llm/framework.rs with LlmProvider trait
  - Implement LlmManager in src/llm/mod.rs
  - Implement provider selection and fallback logic
  - Implement rate limit handling with retry logic
  - _Requirements: 10.1, 10.2, 10.3, 10.4, 10.5, 10.6_

  - [ ]* 18.1 Write property test for LLM request routing
    - **Property 12: LLM request routing**
    - **Validates: Requirements 10.2**

  - [ ]* 18.2 Write property test for provider fallback
    - **Property 13: Provider fallback mechanism**
    - **Validates: Requirements 10.4, 28.2**

  - [ ]* 18.3 Write property test for rate limit retry
    - **Property 14: Rate limit retry success**
    - **Validates: Requirements 10.6**

- [ ] 19. Implement OpenRouter LLM provider
  - Create src/llm/openrouter.rs with OpenRouterProvider
  - Implement API request formatting
  - Implement response parsing
  - Implement streaming support
  - _Requirements: 11.1, 11.2, 11.3, 11.4, 11.5_

  - [ ]* 19.1 Write unit test for OpenRouter integration
    - Test API requests and response parsing
    - _Requirements: 11.1, 11.3_

- [ ] 20. Implement Anthropic Claude LLM provider
  - Create src/llm/claude.rs with ClaudeProvider
  - Implement API request formatting
  - Implement response parsing
  - Implement streaming support
  - _Requirements: 12.1, 12.2, 12.3, 12.4, 12.5_

  - [ ]* 20.1 Write unit test for Claude integration
    - Test API requests and response parsing
    - _Requirements: 12.1, 12.3_

- [ ] 21. Implement OpenAI LLM provider
  - Create src/llm/openai.rs with OpenAIProvider
  - Implement API request formatting
  - Implement response parsing
  - Implement streaming support
  - _Requirements: 13.1, 13.2, 13.3, 13.4, 13.5_

  - [ ]* 21.1 Write unit test for OpenAI integration
    - Test API requests and response parsing
    - _Requirements: 13.1, 13.3_

- [ ] 22. Implement additional LLM providers
  - Create src/llm/gemini.rs with GeminiProvider
  - Create src/llm/zhipu.rs with ZhipuProvider
  - Create src/llm/deepseek.rs with DeepSeekProvider
  - Create src/llm/groq.rs with GroqProvider
  - _Requirements: 14.1, 14.2, 14.3, 14.4, 14.5_

  - [ ]* 22.1 Write unit tests for additional providers
    - Test each provider's API integration
    - _Requirements: 14.1, 14.2, 14.3, 14.4_

- [ ] 23. Checkpoint - Verify LLM providers
  - Ensure all tests pass
  - Verify all providers can generate responses
  - Ask the user if questions arise

## Phase 5: Tools System (Weeks 11-13)

- [ ] 24. Implement tool framework and abstractions
  - Create src/tools/framework.rs with Tool trait
  - Implement ToolRegistry in src/tools/mod.rs
  - Implement tool execution pipeline
  - _Requirements: 16.1, 17.1, 18.1, 19.1, 20.1, 21.1_

- [ ] 25. Implement web search tools
  - Create src/tools/web_search.rs with WebSearchTool
  - Implement Brave Search integration
  - Implement DuckDuckGo integration
  - Implement result parsing and formatting
  - _Requirements: 15.1, 15.2, 15.3, 15.4, 15.5_

  - [ ]* 25.1 Write unit test for web search
    - Test search result parsing
    - _Requirements: 15.3, 15.4_

- [ ] 26. Implement filesystem tool
  - Create src/tools/filesystem.rs with FilesystemTool
  - Implement file reading with size limits
  - Implement file writing with permission validation
  - Implement directory listing
  - Implement path validation
  - _Requirements: 16.1, 16.2, 16.3, 16.4, 16.5_

  - [ ]* 26.1 Write property test for file round-trip
    - **Property 15: File read-write round-trip**
    - **Validates: Requirements 16.1, 16.2**

  - [ ]* 26.2 Write property test for path validation
    - **Property 16: Path validation enforcement**
    - **Validates: Requirements 16.4**

- [ ] 27. Implement shell execution tool
  - Create src/tools/shell.rs with ShellTool
  - Implement command execution with timeout
  - Implement output capture (stdout and stderr)
  - Implement command whitelist validation
  - _Requirements: 17.1, 17.2, 17.3, 17.4, 17.5_

  - [ ]* 27.1 Write property test for command whitelist
    - **Property 17: Shell command whitelist enforcement**
    - **Validates: Requirements 17.3**

  - [ ]* 27.2 Write property test for command timeout
    - **Property 18: Command timeout enforcement**
    - **Validates: Requirements 17.4**

- [ ] 28. Implement web access tool
  - Create src/tools/web_access.rs with WebAccessTool
  - Implement HTTP GET requests
  - Implement redirect following
  - Implement content extraction
  - Implement robots.txt respect
  - _Requirements: 18.1, 18.2, 18.3, 18.4, 18.5_

  - [ ]* 28.1 Write unit test for web access
    - Test HTTP requests and content extraction
    - _Requirements: 18.1, 18.3_

- [ ] 29. Implement hardware interface tools
  - Create src/tools/hardware.rs with HardwareTool
  - Implement I2C read/write operations
  - Implement SPI read/write operations
  - Implement device configuration
  - _Requirements: 19.1, 19.2, 19.3, 19.4, 19.5, 19.6_

  - [ ]* 29.1 Write unit test for hardware operations
    - Test I2C and SPI operations
    - _Requirements: 19.1, 19.3_

- [ ] 30. Implement message tool
  - Create src/tools/message.rs with MessageTool
  - Implement message sending to channels
  - Implement channel routing
  - Implement message formatting
  - _Requirements: 20.1, 20.2, 20.3, 20.4_

  - [ ]* 30.1 Write unit test for message tool
    - Test message sending and routing
    - _Requirements: 20.1, 20.2_

- [ ] 31. Implement cron scheduling tool
  - Create src/tools/cron.rs with CronTool
  - Implement cron expression parsing
  - Implement schedule registration and execution
  - Implement timezone-aware scheduling
  - _Requirements: 21.1, 21.2, 21.3, 21.4, 21.5_

  - [ ]* 31.1 Write unit test for cron scheduling
    - Test cron expression parsing and execution
    - _Requirements: 21.1, 21.2_

- [ ] 32. Checkpoint - Verify tools system
  - Ensure all tests pass
  - Verify all tools execute correctly
  - Ask the user if questions arise

## Phase 6: Integration and Optimization (Weeks 14-16)

- [ ] 33. Implement backward compatibility layer
  - Create compatibility tests comparing Go and Rust versions
  - Verify CLI interface matches Go version
  - Verify configuration format compatibility
  - Verify output format compatibility
  - _Requirements: 1.2, 26.1, 26.2, 26.3, 26.4, 26.5_

  - [ ]* 33.1 Write property test for backward compatibility
    - **Property 1: Backward compatibility during transition**
    - **Validates: Requirements 1.2**

- [ ] 34. Implement performance monitoring and optimization
  - Add boot time measurement
  - Add memory usage monitoring
  - Add message processing latency tracking
  - Optimize hot paths based on profiling
  - _Requirements: 27.1, 27.2, 27.3, 27.4, 27.5_

  - [ ]* 34.1 Write property test for boot time
    - **Property 23: Boot time constraint**
    - **Validates: Requirements 27.1**

  - [ ]* 34.2 Write property test for message latency
    - **Property 24: Message processing latency**
    - **Validates: Requirements 27.4**

  - [ ]* 34.3 Write property test for concurrent handling
    - **Property 25: Concurrent message handling**
    - **Validates: Requirements 27.5**

- [ ] 35. Implement error recovery and resilience
  - Implement panic recovery with logging
  - Implement graceful degradation for failed components
  - Implement health check endpoints
  - _Requirements: 28.1, 28.2, 28.3, 28.4, 28.5_

  - [ ]* 35.1 Write property test for graceful shutdown
    - **Property 28: Graceful shutdown completeness**
    - **Validates: Requirements 28.5**

- [ ] 36. Optimize binary size and dependencies
  - Review and minimize dependencies
  - Enable LTO and symbol stripping in release builds
  - Profile binary size
  - Remove unused features
  - _Requirements: 30.5, 30.6_

  - [ ]* 36.1 Write unit test for binary size
    - Verify binary is suitable for embedded deployment
    - _Requirements: 27.3_

- [ ] 37. Create comprehensive integration tests
  - Create end-to-end tests for complete workflows
  - Test channel-to-LLM-to-tool pipelines
  - Test error recovery scenarios
  - Test concurrent operations
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 5.6_

  - [ ]* 37.1 Write integration test for message pipeline
    - Test complete message processing flow
    - _Requirements: 5.1, 5.2, 5.3_

  - [ ]* 37.2 Write integration test for error recovery
    - Test error handling and recovery
    - _Requirements: 5.5, 28.1, 28.2, 28.3_

- [ ] 38. Create documentation and deployment guides
  - Write README with build and deployment instructions
  - Document configuration options
  - Document CLI interface
  - Create troubleshooting guide
  - _Requirements: 26.1, 26.2, 26.3_

- [ ] 39. Final checkpoint - Verify complete system
  - Ensure all tests pass (unit, integration, property)
  - Verify boot time < 1 second
  - Verify memory usage < 10MB
  - Verify binary size suitable for embedded
  - Verify CLI compatibility with Go version
  - Ask the user if questions arise

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Property tests are integrated throughout to catch errors early
- Checkpoints ensure incremental validation and allow course correction
- All tasks build incrementally with no orphaned code
- Testing is comprehensive with both unit and property-based tests

