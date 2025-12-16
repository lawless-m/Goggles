# Rust Project Structure

## Directory Layout

```
gogs-cli/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
│
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root (for testing)
│   ├── cli.rs               # CLI definitions (clap)
│   ├── config.rs            # Configuration management
│   ├── error.rs             # Error types
│   ├── output.rs            # Output formatting
│   │
│   ├── api/
│   │   ├── mod.rs           # Module declarations
│   │   ├── client.rs        # HTTP client wrapper
│   │   ├── issues.rs        # Issue operations
│   │   ├── repos.rs         # Repository operations
│   │   └── types.rs         # API data types
│   │
│   └── commands/
│       ├── mod.rs           # Command dispatcher
│       ├── init.rs          # init command
│       ├── issue.rs         # issue subcommands
│       └── repo.rs          # repo subcommands
│
├── tests/
│   ├── integration_tests.rs
│   ├── fixtures/
│   │   ├── sample_config.toml
│   │   └── api_responses/
│   │       ├── user.json
│   │       ├── repos.json
│   │       └── issues.json
│   └── common/
│       └── mod.rs           # Test utilities
│
└── examples/
    ├── basic_usage.sh
    └── multi_agent_workflow.sh
```

## File Contents

### `Cargo.toml`

```toml
[package]
name = "gogs-cli"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "CLI tool for multi-agent development with Gogs"
license = "MIT"
repository = "https://github.com/yourusername/gogs-cli"

[[bin]]
name = "gog"
path = "src/main.rs"

[dependencies]
# HTTP client
reqwest = { version = "0.11", features = ["json", "blocking"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# CLI
clap = { version = "4.4", features = ["derive", "cargo"] }

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Async runtime
tokio = { version = "1", features = ["full"] }

# Utilities
dirs = "5.0"

[dev-dependencies]
# Testing
wiremock = "0.6"
tempfile = "3.8"
assert_cmd = "2.0"
predicates = "3.0"

[profile.release]
strip = true
lto = true
codegen-units = 1
panic = "abort"
```

### `src/main.rs`

```rust
use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod error;
mod output;
mod api;
mod commands;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::dispatch(cli).await
}
```

### `src/lib.rs`

```rust
// Library root for testing
pub mod cli;
pub mod config;
pub mod error;
pub mod output;
pub mod api;
pub mod commands;
```

### `src/cli.rs`

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gog")]
#[command(about = "Gogs CLI for multi-agent development orchestration")]
#[command(version)]
pub struct Cli {
    /// Profile to use (overrides default)
    #[arg(long, global = true)]
    pub profile: Option<String>,
    
    /// Output in JSON format
    #[arg(long, global = true)]
    pub json: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize configuration
    Init,
    
    /// Issue operations
    #[command(subcommand)]
    Issue(IssueCommand),
    
    /// Repository operations
    #[command(subcommand)]
    Repo(RepoCommand),
}

#[derive(Subcommand)]
pub enum IssueCommand {
    /// List issues
    List {
        /// List issues across all repositories
        #[arg(long)]
        all: bool,
        
        /// Only show open issues
        #[arg(long, conflicts_with = "closed")]
        open: bool,
        
        /// Only show closed issues
        #[arg(long)]
        closed: bool,
        
        /// Specific repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
        
        /// Filter by label
        #[arg(long)]
        label: Vec<String>,
    },
    
    /// Show issue details
    Show {
        /// Issue number
        number: i64,
        
        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },
    
    /// Create a new issue
    Create {
        /// Issue title
        title: String,
        
        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
        
        /// Issue body (reads from stdin if not provided)
        #[arg(long)]
        body: Option<String>,
        
        /// Add labels
        #[arg(long)]
        label: Vec<String>,
    },
    
    /// Add comment to issue
    Comment {
        /// Issue number
        number: i64,
        
        /// Comment text
        text: String,
        
        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },
    
    /// Close an issue
    Close {
        /// Issue number
        number: i64,
        
        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },
    
    /// Reopen an issue
    Reopen {
        /// Issue number
        number: i64,
        
        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },
    
    /// Add label to issue
    Label {
        /// Issue number
        number: i64,
        
        /// Label name
        label: String,
        
        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },
    
    /// Remove label from issue
    Unlabel {
        /// Issue number
        number: i64,
        
        /// Label name
        label: String,
        
        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum RepoCommand {
    /// List repositories
    List,
}
```

### `src/config.rs`

```rust
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub defaults: Defaults,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Defaults {
    pub repo: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub gogs_user: String,
    pub token: String,
    pub role: String,
    pub signature: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        let contents = fs::read_to_string(&path)
            .context(format!("Failed to read config from {:?}", path))?;
        
        let config: Config = toml::from_str(&contents)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let contents = toml::to_string_pretty(self)?;
        fs::write(&path, contents)?;
        
        // Set restrictive permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&path, perms)?;
        }
        
        Ok(())
    }
    
    pub fn config_path() -> Result<PathBuf> {
        if let Ok(path) = std::env::var("GOGS_CONFIG") {
            return Ok(PathBuf::from(path));
        }
        
        let config_dir = dirs::config_dir()
            .context("Failed to determine config directory")?;
        
        Ok(config_dir.join("gogs-cli").join("config.toml"))
    }
    
    pub fn get_profile(&self, name: Option<&str>) -> Result<&Profile> {
        let profile_name = name
            .or(self.defaults.profile.as_deref())
            .unwrap_or("default");
        
        self.profiles.get(profile_name)
            .context(format!("Profile '{}' not found in config", profile_name))
    }
}
```

### `src/error.rs`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GogsError {
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Authentication failed: {0}")]
    AuthError(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl GogsError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::NotFound(_) => 2,
            _ => 1,
        }
    }
}
```

### `src/output.rs`

```rust
use crate::api::types::{Issue, Repository};
use serde_json;

pub enum OutputFormat {
    Human,
    Json,
}

pub fn format_issue_list(issues: Vec<(String, Vec<Issue>)>, format: OutputFormat) -> String {
    match format {
        OutputFormat::Human => format_issues_human(issues),
        OutputFormat::Json => format_issues_json(issues),
    }
}

fn format_issues_human(issues: Vec<(String, Vec<Issue>)>) -> String {
    let mut output = String::new();
    let mut total = 0;
    
    for (repo, repo_issues) in &issues {
        if !repo_issues.is_empty() {
            output.push_str(&format!("\n{}\n", repo));
            
            for issue in repo_issues {
                let labels: Vec<String> = issue.labels.iter()
                    .map(|l| format!("[{}]", l.name))
                    .collect();
                let labels_str = labels.join(" ");
                
                output.push_str(&format!(
                    "  #{}  [{}] {} {}\n",
                    issue.number,
                    issue.state,
                    labels_str,
                    issue.title
                ));
                total += 1;
            }
        }
    }
    
    output.push_str(&format!("\nTotal: {} open issues across {} repos\n", 
        total, issues.len()));
    
    output
}

fn format_issues_json(issues: Vec<(String, Vec<Issue>)>) -> String {
    let flattened: Vec<_> = issues.into_iter()
        .flat_map(|(repo, repo_issues)| {
            repo_issues.into_iter().map(move |mut issue| {
                // Add repo field to each issue
                issue
            })
        })
        .collect();
    
    serde_json::to_string_pretty(&flattened).unwrap()
}

// Similar functions for other output types...
```

### `src/api/mod.rs`

```rust
pub mod client;
pub mod issues;
pub mod repos;
pub mod types;

pub use client::GogsClient;
pub use types::*;
```

### `src/api/client.rs`

```rust
use anyhow::Result;
use reqwest::{Client, Method, Response};
use serde_json::Value;
use std::time::Duration;

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
        
        Self { base_url, token, client }
    }
    
    pub async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<Response> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        
        let mut req = self.client
            .request(method, &url)
            .header("Authorization", format!("token {}", self.token))
            .header("Content-Type", "application/json");
        
        if let Some(body) = body {
            req = req.json(&body);
        }
        
        let resp = req.send().await?;
        
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await?;
            return Err(anyhow::anyhow!("API error {}: {}", status, text));
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
    
    pub async fn delete(&self, path: &str) -> Result<Response> {
        self.request(Method::DELETE, path, None).await
    }
}
```

### `src/api/types.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub description: Option<String>,
    pub private: bool,
    pub html_url: String,
    pub clone_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Issue {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub labels: Vec<Label>,
    pub state: String,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    pub id: i64,
    pub body: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
}
```

### `src/api/issues.rs`

```rust
use super::{GogsClient, Issue, Comment};
use anyhow::Result;
use serde_json::json;

impl GogsClient {
    pub async fn list_issues(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
    ) -> Result<Vec<Issue>> {
        let path = format!("/repos/{}/{}/issues?state={}", owner, repo, state);
        let resp = self.get(&path).await?;
        let issues: Vec<Issue> = resp.json().await?;
        Ok(issues)
    }
    
    pub async fn get_issue(
        &self,
        owner: &str,
        repo: &str,
        number: i64,
    ) -> Result<Issue> {
        let path = format!("/repos/{}/{}/issues/{}", owner, repo, number);
        let resp = self.get(&path).await?;
        let issue: Issue = resp.json().await?;
        Ok(issue)
    }
    
    pub async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: Option<&str>,
        labels: Vec<String>,
    ) -> Result<Issue> {
        let path = format!("/repos/{}/{}/issues", owner, repo);
        let payload = json!({
            "title": title,
            "body": body,
            "labels": labels,
        });
        let resp = self.post(&path, payload).await?;
        let issue: Issue = resp.json().await?;
        Ok(issue)
    }
    
    pub async fn update_issue(
        &self,
        owner: &str,
        repo: &str,
        number: i64,
        state: Option<&str>,
    ) -> Result<Issue> {
        let path = format!("/repos/{}/{}/issues/{}", owner, repo, number);
        let payload = json!({
            "state": state,
        });
        let resp = self.patch(&path, payload).await?;
        let issue: Issue = resp.json().await?;
        Ok(issue)
    }
    
    pub async fn list_comments(
        &self,
        owner: &str,
        repo: &str,
        number: i64,
    ) -> Result<Vec<Comment>> {
        let path = format!("/repos/{}/{}/issues/{}/comments", owner, repo, number);
        let resp = self.get(&path).await?;
        let comments: Vec<Comment> = resp.json().await?;
        Ok(comments)
    }
    
    pub async fn create_comment(
        &self,
        owner: &str,
        repo: &str,
        number: i64,
        body: &str,
    ) -> Result<Comment> {
        let path = format!("/repos/{}/{}/issues/{}/comments", owner, repo, number);
        let payload = json!({ "body": body });
        let resp = self.post(&path, payload).await?;
        let comment: Comment = resp.json().await?;
        Ok(comment)
    }
}
```

### `src/commands/mod.rs`

```rust
use anyhow::Result;
use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::api::GogsClient;

pub mod init;
pub mod issue;
pub mod repo;

pub async fn dispatch(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init => {
            init::handle_init().await
        }
        Commands::Issue(cmd) => {
            let config = Config::load()?;
            let profile = config.get_profile(cli.profile.as_deref())?;
            let client = GogsClient::new(
                config.server.url.clone(),
                profile.token.clone(),
            );
            
            issue::handle(cmd, &client, &config, profile, cli.json).await
        }
        Commands::Repo(cmd) => {
            let config = Config::load()?;
            let profile = config.get_profile(cli.profile.as_deref())?;
            let client = GogsClient::new(
                config.server.url.clone(),
                profile.token.clone(),
            );
            
            repo::handle(cmd, &client, cli.json).await
        }
    }
}
```

### `.gitignore`

```
/target/
Cargo.lock
*.swp
*.swo
*~
.DS_Store
```

### `README.md`

```markdown
# Gogs CLI

Multi-agent development orchestration tool for Gogs issue tracking.

## Installation

```bash
cargo build --release
cp target/release/gog ~/.local/bin/
```

## Quick Start

```bash
# Initialize
gog init

# List issues
gog issue list --all

# Create issue
gog issue create "New feature" --repo owner/project

# Comment
gog issue comment 42 "Working on this" --repo owner/project
```

See full documentation in the docs/ directory.
```

## Build Instructions

```bash
# Development build
cargo build

# Run tests
cargo test

# Release build (optimized)
cargo build --release

# Run locally
cargo run -- --help
```

## Cross-Compilation

For Windows from Linux:
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

For Linux from Windows:
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```
