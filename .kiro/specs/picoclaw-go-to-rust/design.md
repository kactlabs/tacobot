# PicoClaw Rust Conversion Design

## Overview

The PicoClaw Go-to-Rust conversion transforms an ultra-lightweight personal AI Assistant from Go to Rust while maintaining all functionality, performance characteristics, and API compatibility. The design leverages Rust's memory safety guarantees, zero-cost abstractions, and strong type system to ensure reliability on resource-constrained embedded systems.

### Key Design Principles

1. **Memory Safety**: Leverage Rust's ownership system to eliminate entire classes of bugs
2. **Performance**: Maintain <10MB RAM footprint and <1 second boot time
3. **Concurrency**: Use tokio async runtime for efficient I/O multiplexing
4. **Modularity**: Clear separation of concerns with well-defined interfaces
5. **Compatibility**: Maintain API and CLI compatibility with Go version
6. **Testability**: Design for comprehensive property-based and unit testing

### Conversion Phases

**Phase 1: Core Infrastructure** (Weeks 1-2)
- Async runtime setup with tokio
- Configuration management system
- Logging and error handling framework
- Session and state management
- Basic CLI interface

**Phase 2: Authentication and Agent Loop** (Weeks 3-4)
- OAuth2 and PKCE authentication system
- Agent loop implementation
- Context management
- Memory management subsystem
- Device manager

**Phase 3: Channel Integrations** (Weeks 5-7)
- Channel framework and abstractions
- Telegram integration
- Discord integration
- Additional channels (DingTalk, LINE, QQ, WhatsApp)

**Phase 4: LLM Providers** (Weeks 8-10)
- LLM provider framework
- OpenRouter integration
- Claude (Anthropic) integration
- OpenAI integration
- Additional providers (Gemini, Zhipu, DeepSeek, Groq)

**Phase 5: Tools System** (Weeks 11-13)
- Tool framework and abstractions
- Web search tools (Brave, DuckDuckGo)
- Filesystem tool
- Shell execution tool
- Web access tool
- Hardware tools (I2C, SPI)
- Message tool
- Cron scheduling tool

**Phase 6: Integration and Optimization** (Weeks 14-16)
- End-to-end integration testing
- Performance optimization
- Binary size reduction
- Documentation and deployment

## Architecture

### High-Level System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI Interface Layer                      │
│              (Command-line argument parsing)                 │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│                  Configuration Manager                       │
│         (YAML/TOML loading, env var overrides)              │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│                   Async Runtime (tokio)                      │
│              (Task scheduling, I/O multiplexing)             │
└────────────────────┬────────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
┌───────▼──┐  ┌──────▼──┐  ┌─────▼──────┐
│  Channels│  │ Agent   │  │  Tools     │
│Framework │  │ Loop    │  │ Framework  │
└───────┬──┘  └──────┬──┘  └─────┬──────┘
        │            │            │
        └────────────┼────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
┌───────▼──┐  ┌──────▼──┐  ┌─────▼──────┐
│ LLM      │  │ Session │  │ Device     │
│ Framework│  │ Manager │  │ Manager    │
└──────────┘  └─────────┘  └────────────┘
```

### Module Organization

```
picoclaw-rust/
├── src/
│   ├── main.rs                 # Entry point and CLI
│   ├── config/                 # Configuration management
│   │   ├── mod.rs
│   │   ├── loader.rs
│   │   └── schema.rs
│   ├── auth/                   # OAuth2 and PKCE
│   │   ├── mod.rs
│   │   ├── oauth2.rs
│   │   ├── pkce.rs
│   │   └── token_storage.rs
│   ├── agent/                  # Agent loop and context
│   │   ├── mod.rs
│   │   ├── loop.rs
│   │   ├── context.rs
│   │   └── memory.rs
│   ├── channels/               # Channel integrations
│   │   ├── mod.rs
│   │   ├── framework.rs
│   │   ├── telegram.rs
│   │   ├── discord.rs
│   │   ├── dingtalk.rs
│   │   ├── line.rs
│   │   ├── qq.rs
│   │   └── whatsapp.rs
│   ├── llm/                    # LLM provider integrations
│   │   ├── mod.rs
│   │   ├── framework.rs
│   │   ├── openrouter.rs
│   │   ├── claude.rs
│   │   ├── openai.rs
│   │   ├── gemini.rs
│   │   ├── zhipu.rs
│   │   ├── deepseek.rs
│   │   └── groq.rs
│   ├── tools/                  # Tool implementations
│   │   ├── mod.rs
│   │   ├── framework.rs
│   │   ├── web_search.rs
│   │   ├── filesystem.rs
│   │   ├── shell.rs
│   │   ├── web_access.rs
│   │   ├── hardware.rs
│   │   ├── message.rs
│   │   └── cron.rs
│   ├── session/                # Session management
│   │   ├── mod.rs
│   │   ├── manager.rs
│   │   └── store.rs
│   ├── device/                 # Device management
│   │   ├── mod.rs
│   │   ├── manager.rs
│   │   ├── i2c.rs
│   │   └── spi.rs
│   ├── logging/                # Logging and tracing
│   │   ├── mod.rs
│   │   └── setup.rs
│   ├── error/                  # Error types and handling
│   │   ├── mod.rs
│   │   └── types.rs
│   └── lib.rs                  # Library exports
├── tests/
│   ├── integration_tests.rs
│   └── property_tests.rs
├── Cargo.toml
└── README.md
```

## Components and Interfaces

### 1. Configuration Manager

**Purpose**: Load and validate system configuration from files and environment variables

**Key Types**:
```rust
pub struct Config {
    pub agent: AgentConfig,
    pub channels: ChannelsConfig,
    pub llm: LlmConfig,
    pub tools: ToolsConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
}

pub struct AgentConfig {
    pub max_context_size: usize,
    pub timeout_ms: u64,
    pub memory_limit_mb: usize,
}
```

**Key Functions**:
- `Config::from_file(path: &str) -> Result<Config>`
- `Config::from_env() -> Result<Config>`
- `Config::validate() -> Result<()>`

**Dependencies**: serde, serde_yaml, serde_toml, dotenv

### 2. Authentication System

**Purpose**: Implement OAuth2 with PKCE flow for secure external service authentication

**Key Types**:
```rust
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: SystemTime,
}

pub struct PkceChallenge {
    pub code_verifier: String,
    pub code_challenge: String,
}
```

**Key Functions**:
- `generate_pkce_challenge() -> PkceChallenge`
- `exchange_code_for_token(code: &str, config: &OAuthConfig) -> Result<TokenPair>`
- `refresh_access_token(refresh_token: &str) -> Result<TokenPair>`
- `TokenStorage::save(provider: &str, token: &TokenPair) -> Result<()>`
- `TokenStorage::load(provider: &str) -> Result<TokenPair>`

**Dependencies**: tokio, reqwest, sha2, base64

### 3. Agent Loop

**Purpose**: Core message processing engine that orchestrates context management, tool execution, and LLM interaction

**Key Types**:
```rust
pub struct AgentContext {
    pub session_id: String,
    pub user_input: String,
    pub conversation_history: Vec<Message>,
    pub available_tools: Vec<String>,
    pub metadata: ContextMetadata,
}

pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: SystemTime,
}

pub enum MessageRole {
    User,
    Assistant,
    System,
}
```

**Key Functions**:
- `AgentLoop::process_message(input: &str, context: &mut AgentContext) -> Result<String>`
- `AgentLoop::execute_tool(tool_name: &str, args: &str) -> Result<String>`
- `AgentLoop::call_llm(context: &AgentContext) -> Result<String>`
- `AgentLoop::handle_error(error: &Error) -> Result<String>`

**Dependencies**: tokio, tracing

### 4. Channel Framework

**Purpose**: Unified interface for all communication platform integrations

**Key Types**:
```rust
pub trait Channel: Send + Sync {
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    async fn receive_message(&mut self) -> Result<Option<IncomingMessage>>;
    async fn send_message(&self, msg: OutgoingMessage) -> Result<()>;
    fn channel_type(&self) -> ChannelType;
}

pub struct IncomingMessage {
    pub channel_id: String,
    pub user_id: String,
    pub content: String,
    pub timestamp: SystemTime,
}

pub struct OutgoingMessage {
    pub channel_id: String,
    pub user_id: String,
    pub content: String,
    pub formatting: ChannelFormatting,
}
```

**Key Functions**:
- `ChannelManager::register_channel(channel: Box<dyn Channel>) -> Result<()>`
- `ChannelManager::broadcast_message(msg: &OutgoingMessage) -> Result<()>`
- `ChannelManager::handle_reconnection(channel_type: ChannelType) -> Result<()>`

**Dependencies**: tokio, async-trait

### 5. LLM Provider Framework

**Purpose**: Unified interface for language model provider integrations

**Key Types**:
```rust
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse>;
    async fn stream(&self, request: LlmRequest) -> Result<BoxStream<'static, Result<String>>>;
    fn provider_name(&self) -> &str;
}

pub struct LlmRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: usize,
}

pub struct LlmResponse {
    pub content: String,
    pub usage: TokenUsage,
}

pub struct TokenUsage {
    pub input_tokens: usize,
    pub output_tokens: usize,
}
```

**Key Functions**:
- `LlmManager::select_provider(name: &str) -> Result<&dyn LlmProvider>`
- `LlmManager::generate_with_fallback(request: &LlmRequest) -> Result<LlmResponse>`
- `LlmManager::handle_rate_limit(provider: &str) -> Result<()>`

**Dependencies**: tokio, reqwest, futures

### 6. Tool Framework

**Purpose**: Unified interface for tool implementations

**Key Types**:
```rust
pub trait Tool: Send + Sync {
    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

pub struct ToolArgs {
    pub params: HashMap<String, String>,
}

pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub metadata: HashMap<String, String>,
}
```

**Key Functions**:
- `ToolRegistry::register_tool(tool: Box<dyn Tool>) -> Result<()>`
- `ToolRegistry::execute_tool(name: &str, args: &ToolArgs) -> Result<ToolResult>`
- `ToolRegistry::list_available_tools() -> Vec<ToolInfo>`

**Dependencies**: tokio, async-trait

### 7. Session Manager

**Purpose**: Manage conversation sessions and state persistence

**Key Types**:
```rust
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created_at: SystemTime,
    pub last_activity: SystemTime,
    pub messages: Vec<Message>,
    pub metadata: SessionMetadata,
}

pub struct SessionMetadata {
    pub channel: String,
    pub tags: Vec<String>,
    pub custom_data: HashMap<String, String>,
}
```

**Key Functions**:
- `SessionManager::create_session(user_id: &str) -> Result<Session>`
- `SessionManager::load_session(session_id: &str) -> Result<Session>`
- `SessionManager::save_session(session: &Session) -> Result<()>`
- `SessionManager::cleanup_expired_sessions() -> Result<usize>`

**Dependencies**: tokio, serde

### 8. Device Manager

**Purpose**: Manage hardware device discovery and lifecycle

**Key Types**:
```rust
pub struct Device {
    pub id: String,
    pub device_type: DeviceType,
    pub address: String,
    pub status: DeviceStatus,
    pub config: DeviceConfig,
}

pub enum DeviceType {
    I2C,
    SPI,
    GPIO,
}

pub enum DeviceStatus {
    Available,
    Unavailable,
    Error(String),
}
```

**Key Functions**:
- `DeviceManager::discover_devices() -> Result<Vec<Device>>`
- `DeviceManager::register_device(device: Device) -> Result<()>`
- `DeviceManager::get_device(id: &str) -> Result<&Device>`
- `DeviceManager::handle_device_error(id: &str, error: &Error) -> Result<()>`

**Dependencies**: tokio, i2cdev (or equivalent), spidev (or equivalent)

## Data Models

### Message Format

```rust
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: SystemTime,
    pub channel: String,
    pub user_id: String,
}
```

### Configuration Schema

```yaml
agent:
  max_context_size: 8192
  timeout_ms: 5000
  memory_limit_mb: 10

channels:
  telegram:
    enabled: true
    token: "${TELEGRAM_TOKEN}"
  discord:
    enabled: true
    token: "${DISCORD_TOKEN}"

llm:
  default_provider: "openrouter"
  providers:
    openrouter:
      api_key: "${OPENROUTER_API_KEY}"
      model: "meta-llama/llama-2-70b-chat"
    claude:
      api_key: "${ANTHROPIC_API_KEY}"
      model: "claude-3-sonnet"

tools:
  web_search:
    enabled: true
    provider: "brave"
  filesystem:
    enabled: true
    allowed_paths: ["/home/user/data"]
  shell:
    enabled: true
    whitelist: ["ls", "cat", "grep"]
```

### Session Storage Format

Sessions are stored as JSON with the following structure:

```json
{
  "id": "session_123",
  "user_id": "user_456",
  "created_at": "2024-01-15T10:30:00Z",
  "last_activity": "2024-01-15T10:45:00Z",
  "messages": [
    {
      "role": "user",
      "content": "Hello",
      "timestamp": "2024-01-15T10:30:00Z"
    },
    {
      "role": "assistant",
      "content": "Hi there!",
      "timestamp": "2024-01-15T10:30:05Z"
    }
  ],
  "metadata": {
    "channel": "telegram",
    "tags": ["general"],
    "custom_data": {}
  }
}
```

## Correctness Properties

A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.



### Property-Based Testing Overview

Property-based testing (PBT) validates software correctness by testing universal properties across many generated inputs. Each property is a formal specification that should hold for all valid inputs.

#### Core Principles

1. **Universal Quantification**: Every property must contain an explicit "for all" statement
2. **Requirements Traceability**: Each property must reference the requirements it validates
3. **Executable Specifications**: Properties must be implementable as automated tests
4. **Comprehensive Coverage**: Properties should cover all testable acceptance criteria

#### Common Property Patterns

1. **Invariants**: Properties that remain constant despite changes
   - Example: `session.messages.len() >= 0` always holds
   - Example: `memory_usage <= 10MB` always holds

2. **Round Trip Properties**: Combining an operation with its inverse returns to original value
   - Example: `deserialize(serialize(config)) == config`
   - Example: `parse(format(message)) == message`

3. **Idempotence**: Doing an operation twice equals doing it once
   - Example: `cleanup_sessions(); cleanup_sessions() == cleanup_sessions()`
   - Example: `mute_channel(); mute_channel() == mute_channel()`

4. **Metamorphic Properties**: Relationships that must hold between components
   - Example: `filtered_messages.len() <= all_messages.len()`
   - Example: `available_devices.len() >= 0`

5. **Error Conditions**: Bad inputs properly signal errors
   - Example: Invalid config file returns descriptive error
   - Example: Malformed OAuth token returns auth error

### Acceptance Criteria Testing Prework

Before writing correctness properties, I'll analyze each acceptance criterion for testability:

**Requirement 1: Phased Conversion Strategy**
- 1.1 Phase definition: Not testable (architectural planning)
- 1.2 Backward compatibility: Yes - property (can verify both versions produce same output)
- 1.3 Dependency ordering: Not testable (planning artifact)
- 1.4 Phase acceptance criteria: Not testable (planning artifact)
- 1.5 Module identification: Not testable (planning artifact)

**Requirement 2: Core Infrastructure**
- 2.1 Tokio usage: Yes - example (verify tokio is initialized)
- 2.2 Runtime initialization: Yes - property (runtime ready within 100ms)
- 2.3 Error handling: Yes - property (all async errors propagate correctly)
- 2.4 Graceful shutdown: Yes - property (all tasks terminate cleanly)
- 2.5 Task pool: Yes - property (concurrent tasks execute without deadlock)

**Requirement 3: Configuration Management**
- 3.1 YAML/TOML loading: Yes - property (round-trip: load → save → load)
- 3.2 Environment overrides: Yes - property (env vars override file values)
- 3.3 Invalid config error: Yes - example (specific invalid config returns error)
- 3.4 Required settings validation: Yes - property (missing required fields rejected)
- 3.5 Schema compatibility: Yes - property (schema matches Go version)

**Requirement 4: Authentication System**
- 4.1 OAuth2 PKCE: Yes - property (PKCE challenge/verifier are valid)
- 4.2 Code verifier generation: Yes - property (verifier is cryptographically secure)
- 4.3 Token storage: Yes - property (stored tokens can be retrieved)
- 4.4 Token refresh: Yes - property (expired token refreshes successfully)
- 4.5 Concurrent sessions: Yes - property (multiple sessions don't interfere)
- 4.6 Auth failure: Yes - example (specific auth failure returns error)

**Requirement 5: Agent Loop**
- 5.1 Sequential processing: Yes - property (messages processed in order)
- 5.2 Context creation: Yes - property (context contains all required fields)
- 5.3 History retention: Yes - property (history respects size limits)
- 5.4 Result persistence: Yes - property (results persist to session store)
- 5.5 Error handling: Yes - property (errors don't stop processing)
- 5.6 Timeout handling: Yes - property (operations timeout correctly)

**Requirement 6: Channel Framework**
- 6.1 Common interface: Not testable (interface design)
- 6.2 Message normalization: Yes - property (all channels normalize to common format)
- 6.3 Concurrent handling: Yes - property (concurrent messages don't interfere)
- 6.4 Response routing: Yes - property (responses route to correct channel)
- 6.5 Channel formatting: Yes - property (responses respect channel constraints)
- 6.6 Reconnection: Yes - property (reconnection succeeds with backoff)

**Requirement 7-9: Channel Integrations**
- All channel-specific criteria: Yes - example (specific channel operations work)

**Requirement 10: LLM Framework**
- 10.1 Common interface: Not testable (interface design)
- 10.2 Request routing: Yes - property (requests route to correct provider)
- 10.3 Streaming: Yes - property (streaming responses are valid)
- 10.4 Fallback: Yes - property (fallback provider works when primary fails)
- 10.5 Format handling: Yes - property (provider formats handled transparently)
- 10.6 Rate limiting: Yes - property (rate-limited requests retry successfully)

**Requirement 11-14: LLM Providers**
- All provider-specific criteria: Yes - example (specific provider operations work)

**Requirement 15: Web Search**
- 15.1 Brave Search: Yes - example (Brave search returns results)
- 15.2 DuckDuckGo: Yes - example (DuckDuckGo search returns results)
- 15.3 Search execution: Yes - property (search returns valid results)
- 15.4 Result parsing: Yes - property (results parsed correctly)
- 15.5 Fallback: Yes - property (fallback provider works)

**Requirement 16: Filesystem Tool**
- 16.1 File reading: Yes - property (read file returns original content)
- 16.2 File writing: Yes - property (written file can be read back)
- 16.3 Directory listing: Yes - property (listing returns all files)
- 16.4 Path validation: Yes - property (paths outside allowed dirs rejected)
- 16.5 Error handling: Yes - example (file not found returns error)

**Requirement 17: Shell Tool**
- 17.1 Command execution: Yes - property (command output matches expected)
- 17.2 Output capture: Yes - property (stdout and stderr captured)
- 17.3 Whitelist validation: Yes - property (non-whitelisted commands rejected)
- 17.4 Timeout: Yes - property (long-running commands timeout)
- 17.5 Error handling: Yes - example (failed command returns exit code)

**Requirement 18: Web Access**
- 18.1 HTTP GET: Yes - property (GET request returns content)
- 18.2 Redirects: Yes - property (redirects followed correctly)
- 18.3 Content extraction: Yes - property (text extracted from HTML)
- 18.4 robots.txt: Yes - property (robots.txt respected)
- 18.5 Error handling: Yes - example (failed request returns error)

**Requirement 19: Hardware Tools**
- 19.1 I2C read/write: Yes - example (I2C operations work)
- 19.2 I2C config: Yes - property (I2C bus/address configurable)
- 19.3 SPI read/write: Yes - example (SPI operations work)
- 19.4 SPI config: Yes - property (SPI bus/CS/clock configurable)
- 19.5 Error handling: Yes - example (hardware error returns error)
- 19.6 Device registry: Yes - property (devices registered and retrievable)

**Requirement 20-21: Message and Cron Tools**
- All criteria: Yes - example (specific tool operations work)

**Requirement 22: Memory Management**
- 22.1 Memory limit: Yes - property (memory usage <= 10MB)
- 22.2 History eviction: Yes - property (old entries evicted when limit exceeded)
- 22.3 Retention policies: Yes - property (policies enforced correctly)
- 22.4 Cleanup: Yes - property (cleanup reduces memory usage)
- 22.5 Metrics: Yes - example (memory metrics available)

**Requirement 23: Session Management**
- 23.1 Session IDs: Yes - property (session IDs are unique)
- 23.2 Session persistence: Yes - property (sessions persist and load correctly)
- 23.3 Session loading: Yes - property (loaded sessions match saved state)
- 23.4 Session expiry: Yes - property (expired sessions cleaned up)
- 23.5 Metadata: Yes - property (metadata preserved correctly)

**Requirement 24: Device Management**
- 24.1 Device discovery: Yes - example (devices discovered on startup)
- 24.2 Device registration: Yes - property (connected devices registered)
- 24.3 Device removal: Yes - property (disconnected devices removed)
- 24.4 Device config: Yes - property (device config applied correctly)
- 24.5 Error handling: Yes - example (device error reported)

**Requirement 25: Logging**
- 25.1 Event logging: Yes - example (events logged with severity)
- 25.2 Error context: Yes - property (error context captured)
- 25.3 Log levels: Yes - property (log levels filter correctly)
- 25.4 Output targets: Yes - example (logs written to targets)
- 25.5 Graceful shutdown: Yes - property (critical errors trigger shutdown)

**Requirement 26: CLI Compatibility**
- 26.1 CLI arguments: Yes - property (same args as Go version accepted)
- 26.2 Help text: Yes - example (help text matches Go version)
- 26.3 Config paths: Yes - property (same config paths work)
- 26.4 Exit codes: Yes - property (exit codes match Go version)
- 26.5 Output format: Yes - property (output format matches Go version)

**Requirement 27: Performance**
- 27.1 Boot time: Yes - property (boot time < 1 second)
- 27.2 Memory usage: Yes - property (memory usage < 10MB)
- 27.3 Binary size: Yes - example (binary suitable for embedded)
- 27.4 Message latency: Yes - property (message response < 5 seconds)
- 27.5 Concurrency: Yes - property (handles 10+ concurrent messages)

**Requirement 28: Error Recovery**
- 28.1 Channel reconnection: Yes - property (reconnection succeeds with backoff)
- 28.2 Provider fallback: Yes - property (fallback provider works)
- 28.3 Tool error handling: Yes - property (tool errors don't stop agent)
- 28.4 Panic recovery: Yes - property (panics logged and recovered)
- 28.5 Graceful shutdown: Yes - property (shutdown without data loss)

**Requirement 29: Testing**
- 29.1 Unit tests: Not testable (testing infrastructure)
- 29.2 Integration tests: Not testable (testing infrastructure)
- 29.3 Test speed: Yes - property (tests complete in < 30 seconds)
- 29.4 Coverage: Yes - example (coverage > 70%)
- 29.5 Property tests: Not testable (testing infrastructure)

**Requirement 30: Dependencies**
- 30.1 Tokio: Yes - example (tokio used for async)
- 30.2 Serde: Yes - example (serde used for serialization)
- 30.3 Reqwest: Yes - example (reqwest used for HTTP)
- 30.4 Tracing: Yes - example (tracing used for logging)
- 30.5 Minimize deps: Yes - property (binary size reasonable)
- 30.6 Verify deps: Not testable (dependency review)

## Property Reflection

After analyzing all acceptance criteria, I've identified redundancies and consolidated properties:

**Consolidated Properties**:
- Configuration round-trip and schema compatibility can be tested together
- All channel reconnection logic follows same pattern (one property covers all)
- All provider fallback logic follows same pattern (one property covers all)
- All tool error handling follows same pattern (one property covers all)
- Memory management properties can be consolidated into one comprehensive property

**Removed Redundancies**:
- Separate properties for each channel type consolidated into channel framework property
- Separate properties for each LLM provider consolidated into LLM framework property
- Separate properties for each tool consolidated into tool framework property

## Correctness Properties

### Property 1: Configuration Round-Trip Consistency
*For any* valid configuration object, serializing it to YAML/TOML and then deserializing should produce an equivalent configuration object.
**Validates: Requirements 3.1, 3.5**

### Property 2: Environment Variable Override
*For any* configuration setting, if an environment variable is set, it should override the corresponding file-based setting.
**Validates: Requirements 3.2**

### Property 3: PKCE Challenge Validity
*For any* PKCE challenge generated, the code verifier should be cryptographically secure (43-128 characters, URL-safe base64) and the challenge should be a valid SHA256 hash of the verifier.
**Validates: Requirements 4.1, 4.2**

### Property 4: Token Persistence Round-Trip
*For any* valid token pair, storing it and then retrieving it should return an equivalent token pair.
**Validates: Requirements 4.3**

### Property 5: Concurrent Session Isolation
*For any* two concurrent sessions, operations on one session should not affect the state of the other session.
**Validates: Requirements 4.5, 5.3**

### Property 6: Message Processing Order
*For any* sequence of messages, the agent should process them in the order they were received, maintaining message ordering in the conversation history.
**Validates: Requirements 5.1**

### Property 7: Context Completeness
*For any* message processed by the agent, the resulting context should contain all required fields (session_id, user_input, conversation_history, available_tools, metadata).
**Validates: Requirements 5.2**

### Property 8: History Size Enforcement
*For any* conversation history, when the size exceeds the configured limit, the oldest messages should be evicted until the size is within limits.
**Validates: Requirements 5.3, 22.2**

### Property 9: Channel Message Normalization
*For any* message received from any channel, it should be normalized to the common message format containing all required fields (channel_id, user_id, content, timestamp).
**Validates: Requirements 6.2**

### Property 10: Concurrent Channel Operations
*For any* set of concurrent messages from different channels, each message should be processed independently without interference or data corruption.
**Validates: Requirements 6.3**

### Property 11: Response Routing Correctness
*For any* outgoing message, it should be routed to the correct channel based on the channel_id field.
**Validates: Requirements 6.4**

### Property 12: LLM Request Routing
*For any* LLM request, it should be routed to the configured provider and return a valid response containing content and token usage.
**Validates: Requirements 10.2**

### Property 13: Provider Fallback Mechanism
*For any* LLM request when the primary provider fails, the system should automatically fall back to an alternative provider and return a valid response.
**Validates: Requirements 10.4**

### Property 14: Rate Limit Retry Success
*For any* rate-limited request, the system should queue it and retry with exponential backoff until it succeeds.
**Validates: Requirements 10.6**

### Property 15: File Read-Write Round-Trip
*For any* file content, writing it to disk and then reading it back should return the exact same content.
**Validates: Requirements 16.1, 16.2**

### Property 16: Path Validation Enforcement
*For any* file path outside the configured allowed directories, the filesystem tool should reject the operation.
**Validates: Requirements 16.4**

### Property 17: Shell Command Whitelist Enforcement
*For any* command not in the whitelist, the shell tool should reject it and return an error.
**Validates: Requirements 17.3**

### Property 18: Command Timeout Enforcement
*For any* command that exceeds the configured timeout, the system should terminate it and return a timeout error.
**Validates: Requirements 17.4**

### Property 19: Memory Usage Constraint
*For any* system state, the runtime memory usage should remain below 10MB.
**Validates: Requirements 22.1, 27.2**

### Property 20: Session Persistence Round-Trip
*For any* session, saving it to storage and then loading it should return an equivalent session with all messages and metadata intact.
**Validates: Requirements 23.2, 23.3**

### Property 21: Session Uniqueness
*For any* two sessions created, they should have unique session IDs.
**Validates: Requirements 23.1**

### Property 22: Device Registry Consistency
*For any* device registered in the device manager, it should be retrievable by its ID and appear in the list of available devices.
**Validates: Requirements 24.2, 24.3**

### Property 23: Boot Time Constraint
*For any* system startup, the time from process start to ready state should be less than 1 second.
**Validates: Requirements 27.1**

### Property 24: Message Processing Latency
*For any* message processed by the agent (excluding LLM inference), the response should be generated within 5 seconds.
**Validates: Requirements 27.4**

### Property 25: Concurrent Message Handling
*For any* set of 10 or more concurrent messages, the system should process all of them without deadlock or data corruption.
**Validates: Requirements 27.5**

### Property 26: Channel Reconnection Success
*For any* channel that fails to connect, the system should attempt reconnection with exponential backoff and eventually succeed.
**Validates: Requirements 28.1**

### Property 27: Error Isolation
*For any* tool that fails, the error should not prevent the agent loop from processing subsequent messages.
**Validates: Requirements 28.3**

### Property 28: Graceful Shutdown Completeness
*For any* system shutdown, all async tasks should terminate cleanly and no data should be lost.
**Validates: Requirements 28.5**

### Property 29: CLI Argument Compatibility
*For any* command-line arguments accepted by the Go version, the Rust version should accept the same arguments and produce equivalent behavior.
**Validates: Requirements 26.1**

### Property 30: Exit Code Consistency
*For any* error condition, the Rust version should exit with the same exit code as the Go version.
**Validates: Requirements 26.4**

## Error Handling

### Error Type Hierarchy

```rust
pub enum PicoClawError {
    // Configuration errors
    ConfigError(String),
    ConfigValidationError(String),
    
    // Authentication errors
    AuthError(String),
    TokenRefreshError(String),
    
    // Channel errors
    ChannelError(String),
    ChannelConnectionError(String),
    
    // LLM errors
    LlmError(String),
    LlmProviderError(String),
    RateLimitError { retry_after: Duration },
    
    // Tool errors
    ToolError(String),
    ToolExecutionError(String),
    
    // Session errors
    SessionError(String),
    SessionNotFoundError(String),
    
    // Device errors
    DeviceError(String),
    DeviceNotFoundError(String),
    
    // System errors
    IoError(std::io::Error),
    TimeoutError,
    InternalError(String),
}
```

### Error Handling Strategies

1. **Configuration Errors**: Fail fast at startup with descriptive messages
2. **Channel Errors**: Log and attempt reconnection with exponential backoff
3. **LLM Errors**: Fall back to alternative provider or return error to user
4. **Tool Errors**: Catch and return error to agent loop, continue processing
5. **Session Errors**: Log and create new session if needed
6. **Device Errors**: Mark device as unavailable, continue operation
7. **System Errors**: Log with full context, attempt graceful shutdown

## Testing Strategy

### Dual Testing Approach

The system uses both unit testing and property-based testing for comprehensive coverage:

**Unit Tests**:
- Specific examples and edge cases
- Integration points between components
- Error conditions and recovery paths
- Channel-specific behavior
- Provider-specific behavior
- Tool-specific behavior

**Property-Based Tests**:
- Universal properties across all inputs
- Comprehensive input coverage through randomization
- Invariant preservation
- Round-trip consistency
- Idempotence verification
- Concurrent operation safety

### Property-Based Testing Configuration

- **Library**: proptest (Rust property-based testing framework)
- **Iterations**: Minimum 100 iterations per property test
- **Shrinking**: Automatic shrinking of failing examples
- **Tagging**: Each test tagged with feature name and property number

### Test Organization

```
tests/
├── unit/
│   ├── config_tests.rs
│   ├── auth_tests.rs
│   ├── agent_tests.rs
│   ├── channel_tests.rs
│   ├── llm_tests.rs
│   ├── tool_tests.rs
│   ├── session_tests.rs
│   └── device_tests.rs
├── integration/
│   ├── channel_integration_tests.rs
│   ├── llm_integration_tests.rs
│   └── end_to_end_tests.rs
└── property/
    ├── config_properties.rs
    ├── auth_properties.rs
    ├── agent_properties.rs
    ├── channel_properties.rs
    ├── llm_properties.rs
    ├── tool_properties.rs
    ├── session_properties.rs
    ├── device_properties.rs
    └── performance_properties.rs
```

### Test Execution

```bash
# Run all tests
cargo test

# Run only property tests
cargo test --test property_tests

# Run with coverage
cargo tarpaulin --out Html

# Run with specific feature
cargo test --features "telegram discord"
```

## Rust Crate Dependencies

### Core Dependencies

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
toml = "0.8"
reqwest = { version = "0.11", features = ["json", "stream"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
async-trait = "0.1"
futures = "0.3"
sha2 = "0.10"
base64 = "0.21"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
anyhow = "1.0"
dotenv = "0.15"
clap = { version = "4.4", features = ["derive"] }
```

### Optional Dependencies (by feature)

```toml
[features]
telegram = ["teloxide"]
discord = ["serenity"]
i2c = ["i2cdev"]
spi = ["spidev"]
web-search = ["scraper"]

[dependencies]
teloxide = { version = "0.27", optional = true }
serenity = { version = "0.12", optional = true, features = ["client", "gateway", "model"] }
i2cdev = { version = "0.5", optional = true }
spidev = { version = "0.5", optional = true }
scraper = { version = "0.17", optional = true }
```

### Development Dependencies

```toml
[dev-dependencies]
proptest = "1.4"
tokio-test = "0.4"
mockito = "1.2"
criterion = "0.5"
```

## Performance Considerations

### Memory Optimization

1. **String Interning**: Use `Arc<str>` for frequently repeated strings
2. **Message Pooling**: Reuse message buffers where possible
3. **Lazy Loading**: Load configuration sections on demand
4. **Memory Limits**: Enforce hard limits on conversation history
5. **Cleanup**: Periodic cleanup of expired sessions and devices

### Concurrency Optimization

1. **Tokio Task Spawning**: Use `tokio::spawn` for independent operations
2. **Channel Multiplexing**: Use tokio channels for inter-task communication
3. **Connection Pooling**: Reuse HTTP connections via reqwest
4. **Batch Operations**: Group database operations where possible

### Binary Size Optimization

1. **Feature Flags**: Make channel/provider integrations optional
2. **Link-Time Optimization**: Enable LTO in release builds
3. **Strip Symbols**: Strip debug symbols from release binary
4. **Minimal Dependencies**: Avoid heavy dependencies where possible

## Deployment Considerations

### Embedded System Deployment

1. **Cross-Compilation**: Support compilation for ARM targets (armv7, aarch64)
2. **Static Linking**: Link against musl for portable binaries
3. **Binary Size**: Target <20MB for embedded deployment
4. **Resource Usage**: Monitor and optimize for <10MB RAM

### Configuration Management

1. **Environment Variables**: Support all settings via env vars
2. **Configuration Files**: Support YAML/TOML in standard locations
3. **Defaults**: Sensible defaults for all settings
4. **Validation**: Validate configuration at startup

### Monitoring and Observability

1. **Structured Logging**: Use tracing for structured logs
2. **Metrics**: Export metrics for monitoring
3. **Health Checks**: Provide health check endpoints
4. **Error Reporting**: Capture and report errors

