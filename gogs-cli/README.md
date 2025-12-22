# Gogs CLI (gog)

A command-line tool for multi-agent development coordination using Gogs issue tracking.

## What Is This?

`gog` enables multiple AI coding agents (Claude Opus, Sonnet, Haiku, local models like Qwen) to collaborate on software development through a shared Gogs issue tracker. Each agent has its own identity and role, communicating asynchronously via issues and comments.

## Quick Start

```bash
# Build
cargo build --release

# Initialize (interactive setup)
./target/release/gog init

# List all issues across all repos
gog issue list --all

# Create an issue
gog issue create "Implement user auth" --repo owner/project --body "Need OAuth2 support"

# Comment on an issue
gog issue comment 42 "Starting implementation" --repo owner/project
```

## Installation

### From Source

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cd gogs-cli
cargo build --release

# Install (Linux)
cp target/release/gog ~/.local/bin/

# Install (Windows)
copy target\release\gog.exe C:\Users\<user>\bin\
```

### Cross-Compilation

```bash
# Windows from Linux
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Linux from Windows
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

## Configuration

Configuration lives at:
- **Linux:** `~/.config/gogs-cli/config.toml`
- **Windows:** `%APPDATA%\gogs-cli\config.toml`
- **Override:** Set `GOGS_CONFIG` environment variable

### Example Config

```toml
[server]
url = "https://gogs.example.com"

[defaults]
repo = "myorg/main-project"  # Optional default repo
profile = "default"           # Default profile to use

[profiles.default]
gogs_user = "human-dev"
token = "your-api-token-here"
role = "Human Developer"
signature = "[Human]"

[profiles.opus-planning]
gogs_user = "bot-opus"
token = "opus-bot-token"
role = "Architecture and Planning"
signature = "[Opus/Planning]"

[profiles.sonnet-backend]
gogs_user = "bot-sonnet"
token = "sonnet-bot-token"
role = "Backend Implementation"
signature = "[Sonnet/Backend]"

[profiles.haiku-triage]
gogs_user = "bot-haiku"
token = "haiku-bot-token"
role = "Triage and Prioritization"
signature = "[Haiku/Triage]"
```

### Getting API Tokens

1. Log into Gogs as each user (human or bot account)
2. Go to **Settings → Applications → Generate New Token**
3. Copy token to config file
4. Repeat for each agent profile

## Commands

### Issue Operations

```bash
# List issues
gog issue list --all                    # All repos, open issues
gog issue list --repo owner/project     # Specific repo
gog issue list --all --closed           # Closed issues
gog issue list --all --label bug        # Filter by label

# Show issue details (includes comments)
gog issue show 42 --repo owner/project

# Create issue
gog issue create "Title" --repo owner/project
gog issue create "Title" --repo owner/project --body "Description here"
gog issue create "Bug" --repo owner/project --label bug --label urgent

# Comment on issue
gog issue comment 42 "Working on this" --repo owner/project

# Change state
gog issue close 42 --repo owner/project
gog issue reopen 42 --repo owner/project

# Manage labels
gog issue label 42 in-progress --repo owner/project
gog issue unlabel 42 needs-triage --repo owner/project
```

### Repository Operations

```bash
gog repo list                           # List accessible repos
gog repo list --json                    # JSON output
```

### Global Options

```bash
--profile <name>    # Use specific profile (overrides default)
--json              # Output as JSON (for scripting)
--help              # Show help
--version           # Show version
```

## Multi-Agent Workflow

### The Concept

Different AI models have different strengths:
- **Opus:** Complex planning, architecture decisions
- **Sonnet:** Implementation, code generation
- **Haiku:** Quick triage, simple tasks
- **Local models:** Offline work, experimentation

`gog` lets each run with its own identity, communicating through issues.

### Example Workflow

```
1. Human creates issue: "Add user authentication"

2. Planning agent (opus-planning) analyzes:
   $ gog --profile opus-planning issue list --label needs-planning --json
   $ gog --profile opus-planning issue comment 42 "Breaking into subtasks:
     1. OAuth2 flow implementation
     2. Token storage
     3. Session management"
   $ gog --profile opus-planning issue label 42 planned

3. Implementation agent (sonnet-backend) claims work:
   $ gog --profile sonnet-backend issue comment 43 "Claiming OAuth2 subtask"
   $ gog --profile sonnet-backend issue label 43 in-progress
   # ... does work in sandbox VM ...
   $ gog --profile sonnet-backend issue comment 43 "Implementation complete"
   $ gog --profile sonnet-backend issue label 43 needs-review

4. Human reviews and closes:
   $ gog issue show 43 --repo owner/project
   $ gog issue close 43 --repo owner/project
```

### Signatures

Every issue and comment includes the agent's signature:

```
[Opus/Planning] Breaking this into 3 subtasks...
[Sonnet/Backend] Implementation complete, tests passing.
[Haiku/Triage] Labeled as bug, priority high.
```

This creates clear audit trails of who did what.

### Sandboxed Agents

Agents typically run in isolated VMs or containers where they can:
- Check out code safely
- Make changes without affecting production
- Run tests
- Report back via `gog` commands
- Be destroyed after task completion

## JSON Output

All commands support `--json` for scripting:

```bash
# Parse with jq
gog issue list --all --json | jq '.[] | select(.state == "open")'

# Get issue numbers
gog issue list --repo owner/project --json | jq '.[].number'

# Filter by label
gog issue list --all --json | jq '.[] | select(.labels[].name == "bug")'
```

## Exit Codes

- `0` - Success
- `1` - General error (config, API, network)
- `2` - Resource not found

## Troubleshooting

### "Failed to read config"
Run `gog init` to create configuration interactively.

### "Authentication failed"
Check your API token in the config file. Generate a new one if needed.

### "Repository not specified"
Either use `--repo owner/name` or set `defaults.repo` in config.

### "Profile not found"
Check profile name matches one defined in `[profiles.*]` section.

### Connection issues
- Verify server URL in config (no trailing slash)
- Check network connectivity
- Ensure HTTPS certificate is valid

## Development

```bash
# Run tests
cargo test

# Check without building
cargo check

# Lint
cargo clippy

# Format
cargo fmt

# Build docs
cargo doc --open
```

## License

MIT

## See Also

- [Project Overview](../gogs-cli-spec/PROJECT_OVERVIEW.md)
- [CLI Specification](../gogs-cli-spec/CLI_SPECIFICATION.md)
- [Configuration Guide](../gogs-cli-spec/CONFIGURATION.md)
- [API Reference](../gogs-cli-spec/API_REFERENCE.md)
