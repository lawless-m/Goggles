# Requirements Specification

## Functional Requirements

### FR1: Configuration Management

**FR1.1: Initialize Configuration**
- Command: `gog init`
- Interactive prompts for server URL and initial profile
- Creates config directory and default config file
- Validates server connectivity before saving

**FR1.2: Profile Management**
- Support multiple named profiles in config
- Each profile contains: name, gogs_user, token, role, signature
- Ability to set default profile
- Ability to override profile per-command: `gog --profile <name>`

**FR1.3: Configuration Location**
- Linux: `~/.config/gogs-cli/config.toml`
- Windows: `%APPDATA%\gogs-cli\config.toml`
- Support for `GOGS_CONFIG` environment variable to override location

### FR2: Issue Operations

**FR2.1: List Issues**
```
gog issue list [OPTIONS]
  --all          List issues across all user's repos
  --open         Only open issues (default)
  --closed       Only closed issues
  --repo OWNER/REPO    Specific repository
  --label LABEL  Filter by label
  --json         Output as JSON
```

Output format (human-readable):
```
owner/repo1
  #12  [open] [bug] Fix database connection timeout
  #15  [open] [feature] Add export functionality

owner/repo2
  #3   [open] [agent-working] Implement logging
  
Total: 3 open issues across 2 repos
```

**FR2.2: Show Issue Details**
```
gog issue show <id> [OPTIONS]
  --repo OWNER/REPO    Required if not in config default
  --json         Output as JSON
```

Output includes:
- Issue number, title, state
- Created/updated timestamps
- Labels
- Body text
- All comments with author and timestamp

**FR2.3: Create Issue**
```
gog issue create <title> [OPTIONS]
  --repo OWNER/REPO    Required if not in config default
  --body TEXT    Issue body (alternative: read from stdin)
  --label LABEL  Add label (can be repeated)
  --json         Output created issue as JSON
```

**FR2.4: Comment on Issue**
```
gog issue comment <id> <text> [OPTIONS]
  --repo OWNER/REPO    Required if not in config default
  --json         Output comment as JSON
```

Comment format includes profile signature:
```
[ProfileName/Role] Comment text here
```

**FR2.5: Update Issue State**
```
gog issue close <id> [OPTIONS]
gog issue reopen <id> [OPTIONS]
  --repo OWNER/REPO    Required if not in config default
  --json         Output as JSON
```

**FR2.6: Manage Labels**
```
gog issue label <id> <label> [OPTIONS]
gog issue unlabel <id> <label> [OPTIONS]
  --repo OWNER/REPO    Required if not in config default
  --json         Output as JSON
```

### FR3: Repository Operations

**FR3.1: List User Repositories**
```
gog repo list [OPTIONS]
  --json         Output as JSON
```

Needed for `--all` functionality in issue listing.

### FR4: Profile-Specific Behavior

**FR4.1: Automatic Signature Injection**
- When creating issues or comments, automatically prefix with profile signature
- Format: `[ProfileName/Role] user content`
- Example: `[Opus/Planning] Breaking this into 3 subtasks...`

**FR4.2: Profile Metadata in Output**
- When listing issues, optionally show which profile last interacted
- Useful for filtering agent activity

## Non-Functional Requirements

### NFR1: Performance
- Issue list across all repos should complete in < 5 seconds for 10 repos
- Single issue operations should complete in < 1 second
- Response time depends on Gogs server performance

### NFR2: Reliability
- Graceful handling of network failures
- Clear error messages for API errors
- Retry logic for transient failures (optional)

### NFR3: Security
- API tokens stored in config file with restricted permissions (0600)
- No tokens in command-line arguments (shows in process list)
- No tokens in error messages or logs

### NFR4: Usability
- Clear, consistent command structure
- Helpful error messages with suggestions
- `--help` on all commands
- Examples in help text

### NFR5: Portability
- Single binary per platform (no external dependencies)
- Works on Windows 11 and Debian Linux
- No assumption of specific shell (works in cmd, PowerShell, bash)

### NFR6: Maintainability
- Clear code structure
- Comprehensive error types
- Unit tests for core functionality
- Integration tests against mock Gogs API

## Use Cases

### UC1: Human Creates and Triages Issues
**Actor:** Human developer
**Flow:**
1. Human thinks of feature idea
2. `gog issue create "Implement user authentication" --body "Need OAuth2 support"`
3. Later: `gog issue list --all` to review all open work
4. Human adds label: `gog issue label 42 needs-planning`

### UC2: Planning Agent Breaks Down Complex Issue
**Actor:** opus-planning agent
**Flow:**
1. Agent queries: `gog issue list --label needs-planning --json`
2. Agent analyzes issue #42
3. Agent comments with plan: `gog issue comment 42 "Breaking into subtasks: 1) OAuth flow, 2) Token storage, 3) User session management"`
4. Agent creates subtask issues
5. Agent labels original: `gog issue label 42 planned`

### UC3: Implementation Agent Claims and Works on Task
**Actor:** sonnet-backend agent in sandbox VM
**Flow:**
1. VM spins up with gog binary and profile config
2. Agent queries: `gog issue list --label ready-for-implementation --json`
3. Agent claims issue: `gog issue comment 15 "Claiming this task, starting implementation"`
4. Agent labels: `gog issue label 15 agent-working`
5. Agent clones repo, implements feature
6. Agent reports: `gog issue comment 15 "Implementation complete, tests passing"`
7. Agent labels: `gog issue label 15 needs-review`

### UC4: Triage Agent Organizes Work
**Actor:** haiku-triage agent
**Flow:**
1. Agent queries: `gog issue list --all --json`
2. Agent reads recent unlabeled issues
3. Agent adds appropriate labels and priority
4. Agent comments on unclear issues asking for clarification

### UC5: Human Reviews Agent Work
**Actor:** Human developer
**Flow:**
1. Human queries: `gog issue list --label needs-review`
2. Human reviews issue #15 with `gog issue show 15`
3. Human reads implementation comments from agent
4. Human either approves: `gog issue close 15` or requests changes via comment

### UC6: Experimentation with Local Model
**Actor:** qwen-local agent
**Flow:**
1. Human configures qwen-local profile
2. Assigns specific issue types to qwen for testing
3. qwen works on issues, reports back
4. Human compares qwen's performance vs Claude agents
5. Adjusts which agents get which work based on results

## Edge Cases and Error Handling

### E1: Network Failures
- Timeout connecting to Gogs server
- Intermittent connection drops
- **Handling:** Clear error message, suggestion to check connectivity, exit code 1

### E2: Authentication Failures
- Invalid API token
- Token revoked
- Insufficient permissions
- **Handling:** Clear error identifying auth issue, suggestion to check token, exit code 1

### E3: Missing Resources
- Issue doesn't exist
- Repository doesn't exist
- **Handling:** Clear error message, exit code 2 (not found)

### E4: Configuration Issues
- Config file doesn't exist (first run)
- Invalid TOML syntax
- Missing required fields
- **Handling:** Suggest running `gog init`, exit code 1

### E5: Concurrent Modifications
- Issue modified between read and update
- **Handling:** Accept last-write-wins (Gogs behavior), no special handling needed

### E6: Large Result Sets
- User has 100+ repositories
- Issue has 1000+ comments
- **Handling:** Implement pagination, reasonable defaults (first 100 repos, recent 50 comments)

## Future Enhancements (Out of Scope for v1)

- Milestone support
- Assignee management
- Issue templates
- Bulk operations
- Webhook integration
- Local caching for offline work
- Pull request operations
- Repository creation/management
- Advanced search/filtering
- Export/reporting features

These can be added later based on actual usage patterns.
