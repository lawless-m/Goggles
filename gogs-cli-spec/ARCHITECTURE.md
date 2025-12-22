# Architecture Overview

## Design Principles

1. **Single Responsibility:** Tool coordinates; agents provide intelligence
2. **Simplicity:** Do one thing well (issue tracker interaction)
3. **Stateless:** No local state beyond config file
4. **Fail-Fast:** Clear errors rather than silent failures
5. **Composability:** Works in scripts and automation

## Technology Stack

### Language: Rust

**Rationale:**
- Single binary compilation (no runtime dependencies)
- Excellent cross-platform support (Windows, Linux)
- Strong type system reduces bugs
- Great HTTP client libraries
- Fast execution
- Memory safety without garbage collection

### Key Dependencies

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
clap = { version = "4.4", features = ["derive"] }
anyhow = "1.0"
dirs = "5.0"
tokio = { version = "1", features = ["full"] }
```

**Dependency Justification:**
- `reqwest`: HTTP client for Gogs API
- `serde` + `serde_json`: JSON serialization/deserialization
- `toml`: Config file parsing
- `clap`: CLI argument parsing with derive macros
- `anyhow`: Error handling
- `dirs`: Cross-platform config directory discovery
- `tokio`: Async runtime (needed by reqwest)

## Architecture Diagram

```
┌─────────────────────────────────────────────────────┐
│                   CLI Interface                     │
│  (clap argument parsing, subcommands)               │
└────────────────┬────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────┐
│              Command Dispatcher                     │
│  (routes to appropriate handler)                    │
└────────────┬───────────────────────┬────────────────┘
             │                       │
             ▼                       ▼
┌────────────────────┐    ┌──────────────────────────┐
│  Config Manager    │    │   API Client             │
│  - Load config     │    │   - HTTP requests        │
│  - Profile lookup  │    │   - Response parsing     │
│  - Validation      │    │   - Error handling       │
└────────────────────┘    └──────────┬───────────────┘
                                     │
                                     ▼
                          ┌──────────────────────────┐
                          │   Gogs API               │
                          │   (REST endpoints)       │
                          └──────────────────────────┘
```

## Module Structure

```
gogs-cli/
├── src/
│   ├── main.rs              # Entry point, CLI setup
│   ├── cli.rs               # Command definitions (clap)
│   ├── config.rs            # Config loading and profiles
│   ├── api/
│   │   ├── mod.rs           # API client facade
│   │   ├── client.rs        # HTTP client wrapper
│   │   ├── issues.rs        # Issue-related endpoints
│   │   ├── repos.rs         # Repository endpoints
│   │   └── types.rs         # API response types
│   ├── commands/
│   │   ├── mod.rs           # Command dispatcher
│   │   ├── init.rs          # gog init
│   │   ├── issue.rs         # gog issue *
│   │   └── repo.rs          # gog repo *
│   ├── output.rs            # Formatting (human/JSON)
│   └── error.rs             # Error types
├── tests/
│   ├── integration_tests.rs
│   └── fixtures/
├── Cargo.toml
└── README.md
```

## Key Components

### 1. CLI Interface (`cli.rs`)

Uses `clap` derive macros for clean command definition:

```rust
#[derive(Parser)]
#[command(name = "gog")]
#[command(about = "Gogs CLI for multi-agent development")]
struct Cli {
    #[arg(long, global = true)]
    profile: Option<String>,
    
    #[arg(long, global = true)]
    json: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Issue(IssueCommand),
    Repo(RepoCommand),
}
```

### 2. Configuration (`config.rs`)

```rust
#[derive(Deserialize, Serialize)]
struct Config {
    server: ServerConfig,
    defaults: Defaults,
    profiles: HashMap<String, Profile>,
}

#[derive(Deserialize, Serialize)]
struct Profile {
    gogs_user: String,
    token: String,
    role: String,
    signature: String,
}
```

**Responsibilities:**
- Load config from standard location
- Support environment variable override
- Validate required fields
- Profile lookup and selection
- Secure token storage (file permissions)

### 3. API Client (`api/client.rs`)

```rust
struct GogsClient {
    base_url: String,
    token: String,
    client: reqwest::Client,
}

impl GogsClient {
    fn new(base_url: String, token: String) -> Self { ... }
    
    async fn get(&self, path: &str) -> Result<Response> { ... }
    async fn post(&self, path: &str, body: Value) -> Result<Response> { ... }
    async fn patch(&self, path: &str, body: Value) -> Result<Response> { ... }
    async fn delete(&self, path: &str) -> Result<Response> { ... }
}
```

**Responsibilities:**
- Base URL and token management
- HTTP method wrappers
- Authentication header injection
- Response parsing
- Error conversion

### 4. API Endpoints (`api/issues.rs`, `api/repos.rs`)

```rust
impl GogsClient {
    pub async fn list_issues(&self, owner: &str, repo: &str, state: IssueState) 
        -> Result<Vec<Issue>> { ... }
    
    pub async fn get_issue(&self, owner: &str, repo: &str, number: i64) 
        -> Result<Issue> { ... }
    
    pub async fn create_issue(&self, owner: &str, repo: &str, params: CreateIssueParams) 
        -> Result<Issue> { ... }
    
    pub async fn create_comment(&self, owner: &str, repo: &str, number: i64, body: String) 
        -> Result<Comment> { ... }
    
    pub async fn update_issue(&self, owner: &str, repo: &str, number: i64, params: UpdateIssueParams) 
        -> Result<Issue> { ... }
    
    pub async fn list_user_repos(&self) 
        -> Result<Vec<Repository>> { ... }
}
```

### 5. Command Handlers (`commands/`)

Each command has its own module:

```rust
pub async fn handle_issue_list(
    client: &GogsClient,
    config: &Config,
    args: &IssueListArgs,
) -> Result<()> {
    // 1. Determine which repos to query
    // 2. Fetch issues for each repo
    // 3. Format output (human or JSON)
    // 4. Write to stdout
}
```

**Responsibilities:**
- Argument validation
- Orchestrate API calls
- Error handling
- Output formatting

### 6. Output Formatter (`output.rs`)

```rust
pub fn format_issue_list(issues: Vec<IssueWithRepo>, format: OutputFormat) -> String {
    match format {
        OutputFormat::Human => format_human(issues),
        OutputFormat::Json => format_json(issues),
    }
}
```

**Responsibilities:**
- Human-readable tables and lists
- JSON serialization
- Consistent formatting across commands

### 7. Error Handling (`error.rs`)

```rust
#[derive(Debug, thiserror::Error)]
pub enum GogsError {
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Authentication failed: {0}")]
    AuthError(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}
```

**Exit Codes:**
- 0: Success
- 1: General error (config, API, network)
- 2: Resource not found

## Data Flow

### Example: `gog issue list --all`

```
1. CLI parses arguments
   → profile: default, all: true, json: false

2. Config Manager loads config
   → Reads ~/.config/gogs-cli/config.toml
   → Selects "default" profile
   → Extracts server URL and token

3. API Client initialized
   → Creates reqwest client with auth header

4. Command Handler orchestrates:
   a. Call list_user_repos() to get all repos
   b. For each repo, call list_issues(repo, state=open)
   c. Collect all issues with repo metadata

5. Output Formatter formats results
   → Groups by repo
   → Adds labels, numbers, titles
   → Returns formatted string

6. Print to stdout

7. Exit 0
```

## Authentication Flow

```
┌──────────┐
│  Config  │
│   File   │
└────┬─────┘
     │
     │ Load profile
     ▼
┌─────────────┐
│   Profile   │
│   - token   │
└────┬────────┘
     │
     │ Pass to client
     ▼
┌──────────────────────┐
│   HTTP Request       │
│   Header:            │
│   Authorization:     │
│     token <TOKEN>    │
└────┬─────────────────┘
     │
     ▼
┌──────────┐
│   Gogs   │
│   API    │
└──────────┘
```

Gogs uses `Authorization: token <TOKEN>` header for authentication.

## Concurrency Model

- Main operation: async/await with tokio runtime
- API calls are async but sequential (no parallel requests in v1)
- Future optimization: parallel repo queries for `--all` flag

## Configuration File Format

```toml
[server]
url = "https://gogs.example.com"

[defaults]
repo = "owner/default-repo"  # Optional
profile = "default"          # Optional

[profiles.default]
gogs_user = "human-user"
token = "abc123..."
role = "Human Developer"
signature = "[Human]"

[profiles.opus-planning]
gogs_user = "bot-opus"
token = "def456..."
role = "Architecture and Planning"
signature = "[Opus/Planning]"

[profiles.haiku-triage]
gogs_user = "bot-haiku"
token = "ghi789..."
role = "Triage and Prioritization"
signature = "[Haiku/Triage]"
```

## Error Handling Strategy

1. **Propagate Early:** Use `?` operator extensively
2. **Convert at Boundaries:** Convert library errors to GogsError at API boundary
3. **Context:** Use `anyhow::Context` to add context while propagating
4. **User-Facing:** Top-level error handler formats for end users
5. **Exit Codes:** Map error types to appropriate exit codes

## Security Considerations

### Token Storage
- Config file permissions set to 0600 (user read/write only)
- Warning if permissions are too open
- No tokens in command arguments (visible in process list)
- No tokens in error messages or logs

### Input Validation
- Validate repo owner/name format
- Sanitize user input in issue bodies/comments
- URL validation for server config

### HTTPS
- Require HTTPS for Gogs server (configurable warning for HTTP)
- Certificate validation (no self-signed certs by default)

## Performance Considerations

### V1 (Simple)
- Sequential API calls
- No caching
- Direct HTTP requests
- Acceptable for <20 repos, <100 issues total

### Future Optimizations (if needed)
- Parallel repo queries using tokio::spawn
- Response caching with TTL
- Pagination for large result sets
- Connection pooling (reqwest does this already)

## Testing Strategy

### Unit Tests
- Config parsing
- Output formatting
- Error type conversions

### Integration Tests
- Mock Gogs API server (using wiremock or similar)
- Test all command flows
- Test error cases

### Manual Testing
- Real Gogs instance
- Multiple profiles
- Cross-platform (Windows, Linux)

## Build and Distribution

### Development
```bash
cargo build
cargo test
cargo run -- issue list --all
```

### Release
```bash
cargo build --release
strip target/release/gog  # Reduce binary size
```

### Distribution
- Single binary per platform
- GitHub releases with artifacts
- No installers needed (just download and run)

## Deployment Model

### Local Installation
```bash
# Linux
cp gog ~/.local/bin/
chmod +x ~/.local/bin/gog

# Windows
# Copy gog.exe to C:\Users\<user>\bin or add to PATH
```

### Sandbox/VM Deployment
```bash
# Copy binary and config to VM
scp gog config.toml vm:/tmp/
ssh vm
cd /tmp
./gog issue list --all
```

Simple, portable, no installation required.
