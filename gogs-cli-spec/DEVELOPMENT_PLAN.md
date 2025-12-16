# Development Plan

## Implementation Phases

This document outlines a suggested implementation order for building the gogs-cli tool. The phases are designed to build incrementally, with each phase producing something testable.

## Phase 0: Project Setup (Day 1)

### Goals
- Set up Rust project structure
- Install dependencies
- Verify toolchain

### Tasks
```bash
# Create project
cargo new --bin gogs-cli
cd gogs-cli

# Update Cargo.toml with dependencies (see PROJECT_STRUCTURE.md)

# Create module structure
mkdir -p src/{api,commands}
touch src/{cli.rs,config.rs,error.rs,output.rs}
touch src/api/{mod.rs,client.rs,issues.rs,repos.rs,types.rs}
touch src/commands/{mod.rs,init.rs,issue.rs,repo.rs}

# Verify build
cargo build

# Set up git
git init
# Add .gitignore (see PROJECT_STRUCTURE.md)
git add .
git commit -m "Initial project structure"
```

### Deliverable
- Compiling (empty) project
- All modules in place

### Testing
```bash
cargo build
cargo test  # Should pass (no tests yet)
```

---

## Phase 1: Configuration & Error Handling (Day 1-2)

### Goals
- Load and parse config files
- Handle profiles
- Basic error types

### Tasks

**1. Implement `src/error.rs`:**
```rust
// Define GogsError enum
// Implement Display and From traits
// Add exit_code() method
```

**2. Implement `src/config.rs`:**
```rust
// Define Config, Profile, ServerConfig structs
// Implement Config::load()
// Implement Config::save()
// Implement Config::config_path()
// Implement Config::get_profile()
```

**3. Write tests:**
```rust
// Test config parsing (valid and invalid TOML)
// Test profile lookup
// Test defaults
```

### Deliverable
- Config loading works
- Profile selection works
- Tests pass

### Testing
```bash
cargo test config
cargo test error

# Manual test
cat > /tmp/test-config.toml << EOF
[server]
url = "https://test.com"

[profiles.default]
gogs_user = "test"
token = "abc123"
role = "Test"
signature = "[Test]"
EOF

GOGS_CONFIG=/tmp/test-config.toml cargo run -- --help
```

---

## Phase 2: CLI Structure (Day 2)

### Goals
- Define command structure with clap
- Wire up argument parsing
- Basic help output

### Tasks

**1. Implement `src/cli.rs`:**
```rust
// Define Cli struct with clap derive
// Define Commands enum
// Define IssueCommand and RepoCommand enums
// Add all subcommands and options
```

**2. Implement `src/commands/mod.rs`:**
```rust
// Implement dispatch() function
// Route commands to handlers (stubs for now)
```

**3. Update `src/main.rs`:**
```rust
// Parse CLI
// Call dispatch
// Handle errors with exit codes
```

### Deliverable
- `gog --help` works
- `gog issue list --help` shows usage
- Commands recognized but not implemented

### Testing
```bash
cargo run -- --help
cargo run -- issue --help
cargo run -- issue list --help
cargo run -- --profile test issue list --all --json
```

---

## Phase 3: API Client Foundation (Day 3)

### Goals
- HTTP client wrapper
- Authentication
- Basic request/response handling

### Tasks

**1. Implement `src/api/types.rs`:**
```rust
// Define User, Repository, Issue, Comment, Label structs
// Add serde derives
```

**2. Implement `src/api/client.rs`:**
```rust
// Define GogsClient struct
// Implement new()
// Implement request() with auth
// Implement get(), post(), patch(), delete()
// Add error handling
```

**3. Write tests:**
```rust
// Mock API tests with wiremock
// Test auth header injection
// Test error handling
```

### Deliverable
- API client makes authenticated requests
- Can handle success and error responses
- Tests pass

### Testing
```bash
cargo test api::client

# Manual test (requires Gogs instance)
# Add test in src/api/client.rs:
#[tokio::test]
async fn test_real_api() {
    let client = GogsClient::new(
        "https://your-gogs.com".to_string(),
        "your-token".to_string()
    );
    let resp = client.get("/user").await.unwrap();
    println!("{:?}", resp.status());
}
```

---

## Phase 4: Repository Operations (Day 3-4)

### Goals
- List user repositories
- Foundation for --all flag

### Tasks

**1. Implement `src/api/repos.rs`:**
```rust
impl GogsClient {
    pub async fn list_user_repos(&self) -> Result<Vec<Repository>>
}
```

**2. Implement `src/commands/repo.rs`:**
```rust
pub async fn handle(cmd: RepoCommand, client: &GogsClient, json: bool) -> Result<()>
// Handle RepoCommand::List
```

**3. Implement `src/output.rs` (partial):**
```rust
pub fn format_repo_list(repos: Vec<Repository>, format: OutputFormat) -> String
```

**4. Write tests:**
```rust
// Test repo listing
// Test output formatting
```

### Deliverable
- `gog repo list` works
- Shows all accessible repositories

### Testing
```bash
# With real Gogs instance
gog repo list
gog repo list --json | jq
```

---

## Phase 5: Basic Issue Operations (Day 4-5)

### Goals
- List issues (single repo)
- Get issue details
- Create issues

### Tasks

**1. Implement `src/api/issues.rs`:**
```rust
impl GogsClient {
    pub async fn list_issues(&self, owner: &str, repo: &str, state: &str) -> Result<Vec<Issue>>
    pub async fn get_issue(&self, owner: &str, repo: &str, number: i64) -> Result<Issue>
    pub async fn create_issue(...) -> Result<Issue>
}
```

**2. Implement `src/commands/issue.rs` (partial):**
```rust
pub async fn handle(cmd: IssueCommand, ...) -> Result<()>
// Handle IssueCommand::List (single repo)
// Handle IssueCommand::Show
// Handle IssueCommand::Create
```

**3. Implement signature injection:**
```rust
// Prepend profile signature to issue body
let body_with_sig = format!("{} {}", profile.signature, body);
```

**4. Implement output formatting:**
```rust
// format_issue_list()
// format_issue_detail()
```

### Deliverable
- Can list issues in a repo
- Can show issue details
- Can create issues with signature

### Testing
```bash
# With real Gogs
gog issue list --repo owner/test-repo
gog issue show 1 --repo owner/test-repo
gog issue create "Test issue" --repo owner/test-repo --body "Test body"
gog issue list --repo owner/test-repo --json | jq
```

---

## Phase 6: Issue Updates (Day 5-6)

### Goals
- Close/reopen issues
- Add comments
- Signature in comments

### Tasks

**1. Extend `src/api/issues.rs`:**
```rust
impl GogsClient {
    pub async fn update_issue(...) -> Result<Issue>
    pub async fn list_comments(...) -> Result<Vec<Comment>>
    pub async fn create_comment(...) -> Result<Comment>
}
```

**2. Extend `src/commands/issue.rs`:**
```rust
// Handle IssueCommand::Close
// Handle IssueCommand::Reopen
// Handle IssueCommand::Comment
```

### Deliverable
- Can close/reopen issues
- Can add comments with signature

### Testing
```bash
gog issue comment 1 "Test comment" --repo owner/test-repo
gog issue show 1 --repo owner/test-repo  # Verify comment appears
gog issue close 1 --repo owner/test-repo
gog issue list --repo owner/test-repo --closed
gog issue reopen 1 --repo owner/test-repo
```

---

## Phase 7: Label Management (Day 6)

### Goals
- Add/remove labels from issues

### Tasks

**1. Extend `src/api/issues.rs`:**
```rust
impl GogsClient {
    pub async fn list_labels(...) -> Result<Vec<Label>>
    pub async fn add_label_to_issue(...) -> Result<Vec<Label>>
    pub async fn remove_label_from_issue(...) -> Result<()>
}
```

**2. Extend `src/commands/issue.rs`:**
```rust
// Handle IssueCommand::Label
// Handle IssueCommand::Unlabel
```

### Deliverable
- Can add/remove labels

### Testing
```bash
gog issue label 1 bug --repo owner/test-repo
gog issue show 1 --repo owner/test-repo  # Verify label
gog issue unlabel 1 bug --repo owner/test-repo
```

---

## Phase 8: Multi-Repo Operations (Day 7)

### Goals
- List issues across all repos (--all flag)
- Efficient querying

### Tasks

**1. Extend `src/commands/issue.rs`:**
```rust
async fn handle_list_all(client: &GogsClient, ...) -> Result<()> {
    // Get all repos
    let repos = client.list_user_repos().await?;
    
    // Query each repo
    let mut all_issues = Vec::new();
    for repo in repos {
        let issues = client.list_issues(...).await?;
        all_issues.push((repo.full_name, issues));
    }
    
    // Format and output
    println!("{}", format_issue_list(all_issues, format));
}
```

**2. Update output formatting:**
```rust
// Group issues by repo
// Show repo headers
// Show total count
```

### Deliverable
- `gog issue list --all` works across multiple repos

### Testing
```bash
# Create multiple test repos and issues
gog issue list --all
gog issue list --all --json | jq 'group_by(.repo)'
gog issue list --all --label bug
```

---

## Phase 9: Init Command (Day 7)

### Goals
- Interactive configuration setup
- Better first-run experience

### Tasks

**1. Implement `src/commands/init.rs`:**
```rust
pub async fn handle_init() -> Result<()> {
    // Prompt for server URL
    // Test connectivity
    // Prompt for profile details
    // Create config file
    // Set permissions
    // Confirm success
}
```

**2. Add interactive prompts:**
```rust
use std::io::{self, Write};

fn prompt(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
```

### Deliverable
- `gog init` creates working config

### Testing
```bash
# Remove config
rm ~/.config/gogs-cli/config.toml

# Run init
gog init
# Answer prompts

# Verify
cat ~/.config/gogs-cli/config.toml
gog repo list
```

---

## Phase 10: Polish & Documentation (Day 8)

### Goals
- Clean up error messages
- Improve help text
- Add examples
- Final testing

### Tasks

**1. Improve error messages:**
```rust
// Add context to errors
// Suggest fixes
// Format nicely
```

**2. Add command examples:**
```rust
#[command(
    name = "list",
    about = "List issues",
    long_about = "List issues from repositories. Examples:\n  \
        gog issue list --all\n  \
        gog issue list --repo owner/project\n  \
        gog issue list --all --label bug"
)]
```

**3. Write README.md:**
```markdown
# Installation
# Quick Start
# Examples
# Documentation
```

**4. Final testing:**
- Run through all manual test checklist
- Test on both Windows and Linux
- Test with real multi-agent workflow

### Deliverable
- Production-ready v0.1.0
- Complete documentation
- Tested on both platforms

---

## Phase 11: Optimization (Optional)

### Goals
- Improve performance
- Add caching
- Parallel queries

### Tasks

**1. Parallel repo queries:**
```rust
use tokio::task;

let handles: Vec<_> = repos.iter()
    .map(|repo| {
        let client = client.clone();
        let repo = repo.clone();
        task::spawn(async move {
            client.list_issues(&repo.owner.username, &repo.name, "open").await
        })
    })
    .collect();

let results = futures::future::join_all(handles).await;
```

**2. Response caching:**
```rust
// Cache repo list for 5 minutes
// Cache labels per repo
```

**3. Measure improvements:**
```bash
time gog issue list --all  # Before
# Implement optimizations
time gog issue list --all  # After
```

---

## Success Criteria

After all phases:
- [ ] All commands work on Linux
- [ ] All commands work on Windows  
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing checklist complete
- [ ] Documentation written
- [ ] Multi-agent workflow tested
- [ ] Performance acceptable (<5s for --all with 10 repos)

## Development Tips

### Daily Workflow
```bash
# Start of day
git pull
cargo test

# During development
cargo check  # Fast compile check
cargo clippy  # Linting
cargo test -- --nocapture  # Test with output

# Before commit
cargo test
cargo clippy -- -D warnings
cargo fmt
git add .
git commit -m "Descriptive message"
```

### Debugging
```bash
# Run with logs
RUST_LOG=debug cargo run -- issue list --all

# Run specific test with logs
RUST_LOG=debug cargo test test_name -- --nocapture

# Check for common issues
cargo clippy
```

### Cross-Platform Testing

**Linux → Windows testing:**
```bash
# On Linux, build for Windows
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Copy to Windows and test
scp target/x86_64-pc-windows-gnu/release/gog.exe windows-machine:
```

**Windows → Linux testing:**
```powershell
# On Windows, build for Linux
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu

# Copy to Linux and test
scp target/x86_64-unknown-linux-gnu/release/gog linux-machine:
```

## Known Challenges

### Challenge 1: Label Operations
Gogs API for labels can be inconsistent across versions. May need to:
- Try label by ID first
- Fall back to label by name
- Cache label ID mappings

**Solution:** Implement flexible label resolution.

### Challenge 2: Pagination
Large repos with many issues need pagination.

**Solution:** Start with reasonable limits (50 issues per repo), add pagination if needed.

### Challenge 3: Unicode
Issue titles/bodies with Unicode characters.

**Solution:** Rust handles this well, but test specifically with emoji and non-Latin scripts.

### Challenge 4: Windows Paths
Path handling on Windows (backslashes, %APPDATA%).

**Solution:** Use `dirs` crate, test thoroughly on Windows.

## After v0.1.0

Future enhancements (not part of initial development):
- Milestone support
- Assignee management
- Pull request operations
- Bulk operations
- Web hooks
- Local caching
- Shell completion scripts
- More output formats (table, CSV)

These can be added based on actual usage and feedback.
