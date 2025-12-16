# Testing Strategy

## Testing Philosophy

The gogs-cli tool should be thoroughly tested at multiple levels:
1. **Unit tests** for individual functions and modules
2. **Integration tests** against mock API
3. **Manual testing** against real Gogs instance
4. **Cross-platform testing** on Windows and Linux

## Test Pyramid

```
         Manual Testing
       (Real Gogs, Cross-platform)
              △
             ╱ ╲
            ╱   ╲
           ╱     ╲
          ╱       ╲
         ╱         ╲
    Integration Tests
   (Mock API Server)
        △
       ╱ ╲
      ╱   ╲
     ╱     ╲
    ╱       ╲
   ╱_________╲
   Unit Tests
```

## Unit Tests

### What to Test

**Configuration (`config.rs`)**
- Config file parsing (valid and invalid TOML)
- Profile lookup (existing and missing)
- Default handling
- Path resolution
- Environment variable overrides

**Output Formatting (`output.rs`)**
- Human-readable formatting
- JSON formatting
- Edge cases (empty results, long titles)

**Error Handling (`error.rs`)**
- Error type conversions
- Exit code mapping
- Error message formatting

**API Types (`api/types.rs`)**
- Serialization/deserialization
- Field presence/absence

### Example Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_parse_valid() {
        let toml = r#"
            [server]
            url = "https://gogs.example.com"
            
            [profiles.default]
            gogs_user = "test"
            token = "abc123"
            role = "Test"
            signature = "[Test]"
        "#;
        
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.server.url, "https://gogs.example.com");
        assert!(config.profiles.contains_key("default"));
    }
    
    #[test]
    fn test_config_missing_token() {
        let toml = r#"
            [server]
            url = "https://gogs.example.com"
            
            [profiles.default]
            gogs_user = "test"
            role = "Test"
            signature = "[Test]"
        "#;
        
        let result: Result<Config, _> = toml::from_str(toml);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_profile_lookup_default() {
        let config = create_test_config();
        let profile = config.get_profile(None).unwrap();
        assert_eq!(profile.gogs_user, "default-user");
    }
    
    #[test]
    fn test_profile_lookup_named() {
        let config = create_test_config();
        let profile = config.get_profile(Some("opus-planning")).unwrap();
        assert_eq!(profile.signature, "[Opus/Planning]");
    }
    
    #[test]
    fn test_output_format_human() {
        let issues = vec![
            create_test_issue(1, "Bug fix", "open"),
            create_test_issue(2, "Feature", "open"),
        ];
        
        let output = format_issue_list(
            vec![("owner/repo".to_string(), issues)],
            OutputFormat::Human
        );
        
        assert!(output.contains("#1"));
        assert!(output.contains("Bug fix"));
        assert!(output.contains("Total: 2"));
    }
}
```

### Running Unit Tests

```bash
# Run all tests
cargo test

# Run specific module
cargo test config

# Run with output
cargo test -- --nocapture

# Run with coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

## Integration Tests

### Mock API Server

Use `wiremock` to create a mock Gogs API server for integration testing.

**Setup:**
```rust
// tests/integration_tests.rs
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, header};

#[tokio::test]
async fn test_list_issues() {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Set up mock response
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/owner/repo/issues"))
        .and(header("Authorization", "token test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 1,
                "number": 42,
                "title": "Test issue",
                "body": "Test body",
                "state": "open",
                "labels": [],
                "user": {"id": 1, "username": "test"},
                "comments": 0,
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z",
                "html_url": "https://gogs.example.com/owner/repo/issues/42"
            }
        ])))
        .mount(&mock_server)
        .await;
    
    // Create client pointing to mock server
    let client = GogsClient::new(
        mock_server.uri(),
        "test-token".to_string()
    );
    
    // Test the API call
    let issues = client.list_issues("owner", "repo", "open").await.unwrap();
    
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].number, 42);
    assert_eq!(issues[0].title, "Test issue");
}
```

### Test Fixtures

Store sample API responses in `tests/fixtures/`:

```
tests/
├── fixtures/
│   ├── api_responses/
│   │   ├── user.json
│   │   ├── repos.json
│   │   ├── issues_open.json
│   │   ├── issues_closed.json
│   │   ├── issue_single.json
│   │   ├── comments.json
│   │   └── labels.json
│   └── sample_config.toml
```

**Example fixture (`tests/fixtures/api_responses/issues_open.json`):**
```json
[
  {
    "id": 100,
    "number": 42,
    "title": "Fix database connection",
    "body": "Connection timeouts...",
    "user": {"id": 1, "username": "alice"},
    "labels": [{"id": 1, "name": "bug", "color": "fc2929"}],
    "state": "open",
    "comments": 3,
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-16T14:22:00Z",
    "html_url": "https://gogs.example.com/owner/repo/issues/42"
  }
]
```

### Integration Test Scenarios

**Test scenarios to cover:**

1. **List issues**
   - Empty repository
   - Single issue
   - Multiple issues
   - Pagination

2. **Create issue**
   - With body
   - Without body
   - With labels
   - Invalid repository

3. **Update issue**
   - Close issue
   - Reopen issue
   - Change state

4. **Comments**
   - Create comment
   - List comments
   - With signature injection

5. **Labels**
   - Add label
   - Remove label
   - Non-existent label

6. **Authentication**
   - Valid token
   - Invalid token
   - Missing token

7. **Error handling**
   - Network timeout
   - 404 not found
   - 500 server error
   - Invalid JSON response

### Running Integration Tests

```bash
# Run integration tests only
cargo test --test integration_tests

# Run with logging
RUST_LOG=debug cargo test --test integration_tests

# Run specific test
cargo test test_list_issues
```

## End-to-End Tests

### CLI Testing with `assert_cmd`

Test the actual CLI binary:

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_init_command() {
    let temp = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("gog").unwrap();
    cmd.env("GOGS_CONFIG", temp.path().join("config.toml"))
        .arg("init")
        .write_stdin("https://gogs.example.com\ndefault\nuser\ntoken\nTest\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration saved"));
}

#[test]
fn test_list_without_config() {
    let temp = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("gog").unwrap();
    cmd.env("GOGS_CONFIG", temp.path().join("nonexistent.toml"))
        .arg("issue")
        .arg("list")
        .arg("--all")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read config"));
}
```

## Manual Testing Checklist

### Initial Setup
- [ ] `gog init` creates config file
- [ ] Config file has correct permissions (0600)
- [ ] Invalid server URL gives clear error
- [ ] Invalid token gives auth error

### Basic Operations
- [ ] `gog issue list --all` shows issues across repos
- [ ] `gog issue list --repo owner/repo` shows repo issues
- [ ] `gog issue show 42 --repo owner/repo` displays details
- [ ] `gog issue create "Title" --repo owner/repo` creates issue
- [ ] `gog issue comment 42 "Text" --repo owner/repo` adds comment
- [ ] `gog issue close 42 --repo owner/repo` closes issue

### Profile Functionality
- [ ] `--profile opus-planning` uses correct profile
- [ ] Signature appears in created issues
- [ ] Signature appears in comments
- [ ] Invalid profile gives clear error

### Output Formats
- [ ] Human-readable output is clean and organized
- [ ] `--json` produces valid JSON
- [ ] JSON can be piped to `jq`
- [ ] Long titles don't break formatting

### Edge Cases
- [ ] Empty repository (no issues) handled gracefully
- [ ] Repository with 100+ issues (pagination)
- [ ] Issue with 100+ comments
- [ ] Unicode in titles and bodies
- [ ] Network timeout handled gracefully
- [ ] Invalid repo name gives error

### Cross-Platform
**Linux:**
- [ ] Binary runs on Debian
- [ ] Config location correct (~/.config/gogs-cli/)
- [ ] File permissions work correctly

**Windows:**
- [ ] Binary runs on Windows 11
- [ ] Config location correct (%APPDATA%\gogs-cli\)
- [ ] Paths with spaces work

## Test Coverage Goals

Target coverage for v1:
- **Unit tests:** >80% line coverage
- **Integration tests:** All API endpoints covered
- **E2E tests:** All commands smoke tested
- **Manual:** Full workflow tested on both platforms

## Continuous Testing

### Pre-commit
```bash
#!/bin/bash
# .git/hooks/pre-commit

cargo test --quiet
if [ $? -ne 0 ]; then
    echo "Tests failed, commit aborted"
    exit 1
fi

cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "Clippy failed, commit aborted"
    exit 1
fi
```

### CI Pipeline (GitHub Actions)

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Run tests
      run: cargo test --verbose
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
```

## Test Data Management

### Creating Test Data

For manual testing against real Gogs:

```bash
# Create test repository
curl -X POST https://gogs.example.com/api/v1/user/repos \
  -H "Authorization: token $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "test-repo", "description": "For testing gogs-cli"}'

# Create test issues
for i in {1..10}; do
  gog issue create "Test issue $i" --repo owner/test-repo --body "Test body $i"
done

# Add various labels
gog issue label 1 bug --repo owner/test-repo
gog issue label 2 feature --repo owner/test-repo
gog issue label 3 high-priority --repo owner/test-repo
```

### Cleanup

```bash
# Delete test issues
for i in {1..10}; do
  gog issue close $i --repo owner/test-repo
done

# Or delete entire test repository via Gogs UI
```

## Debugging Failed Tests

### Verbose test output
```bash
cargo test -- --nocapture --test-threads=1
```

### Inspect mock server requests
```rust
// In test
println!("Mock server received: {:?}", mock_server.received_requests().await);
```

### Enable debug logging
```bash
RUST_LOG=debug cargo test test_name
```

## Performance Testing

Basic performance benchmarks:

```bash
# Time listing all issues
time gog issue list --all

# Time creating issue
time gog issue create "Test" --repo owner/repo

# Measure with many repos (10+)
# Should complete in reasonable time (<5s for 10 repos)
```

For detailed profiling:
```bash
cargo install flamegraph
cargo flamegraph -- issue list --all
```

## Regression Testing

Before each release:
1. Run full test suite on both platforms
2. Test against real Gogs instance
3. Verify all documented commands work
4. Check error messages are helpful
5. Validate JSON output with real tools (jq)

## Test Documentation

Each test should have:
- Clear name describing what it tests
- Comment explaining why it's important
- Expected behavior documented
- Edge cases noted

**Example:**
```rust
/// Test that profile signature is correctly prepended to issue body
/// This is critical for agent identification in the issue tracker
#[test]
fn test_signature_injection_create_issue() {
    // Setup profile with signature
    let profile = Profile {
        signature: "[Opus/Planning]".to_string(),
        // ... other fields
    };
    
    // Create issue body
    let body = "Original content";
    let with_signature = format!("{} {}", profile.signature, body);
    
    // Verify signature is prepended
    assert!(with_signature.starts_with("[Opus/Planning]"));
    assert!(with_signature.contains("Original content"));
}
```
