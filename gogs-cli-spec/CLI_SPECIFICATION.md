# CLI Command Specification

## Global Options

These options apply to all commands:

```
--profile <name>    Use specified profile instead of default
--json              Output in JSON format
--help              Show help for command
--version           Show version information
```

## Commands

### `gog init`

Initialize configuration interactively.

**Usage:**
```bash
gog init
```

**Behavior:**
1. Prompts for Gogs server URL
2. Validates connectivity (tries to reach /api/v1)
3. Prompts for initial profile name (default: "default")
4. Prompts for Gogs username
5. Prompts for API token
6. Prompts for role description
7. Generates signature from profile name
8. Creates config directory if needed
9. Writes config file with 0600 permissions
10. Confirms success

**Example Session:**
```
$ gog init
Gogs server URL: https://gogs.example.com
Testing connection... OK
Profile name [default]: default
Gogs username: myuser
API token: abc123def456
Role description: Human Developer
✓ Configuration saved to /home/user/.config/gogs-cli/config.toml
```

**Exit Codes:**
- 0: Success
- 1: Failed to connect to server, invalid input, or write error

---

### `gog issue list`

List issues from repositories.

**Usage:**
```bash
gog issue list [OPTIONS]
```

**Options:**
```
--all                 List issues across all user's repositories
--open                Only show open issues (default)
--closed              Only show closed issues
--repo <owner/repo>   Specific repository (required if no default in config and not using --all)
--label <label>       Filter by label (can be repeated)
```

**Examples:**

List all open issues across all repos:
```bash
gog issue list --all
```

List closed issues in specific repo:
```bash
gog issue list --repo owner/myproject --closed
```

List issues with specific label:
```bash
gog issue list --all --label bug
```

JSON output for scripting:
```bash
gog issue list --all --json | jq '.[] | select(.labels | contains(["needs-review"]))'
```

**Output Format (Human):**
```
owner/repo1
  #12  [open] [bug] [high] Fix database connection timeout
  #15  [open] [feature] Add export functionality

owner/repo2
  #3   [open] [agent-working] Implement logging
  #8   [open] [blocked] Need schema design

Total: 4 open issues across 2 repos
```

**Output Format (JSON):**
```json
[
  {
    "repo": "owner/repo1",
    "number": 12,
    "title": "Fix database connection timeout",
    "state": "open",
    "labels": ["bug", "high"],
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-16T14:22:00Z",
    "url": "https://gogs.example.com/owner/repo1/issues/12"
  }
]
```

**Exit Codes:**
- 0: Success
- 1: Configuration error, API error, or network error

---

### `gog issue show`

Show detailed information about a specific issue.

**Usage:**
```bash
gog issue show <number> [OPTIONS]
```

**Options:**
```
--repo <owner/repo>   Repository (required if no default in config)
```

**Example:**
```bash
gog issue show 42 --repo owner/myproject
```

**Output Format (Human):**
```
Issue #42: Implement user authentication
State: open
Created: 2024-01-15 10:30:00 by alice
Updated: 2024-01-16 14:22:00
Labels: feature, needs-planning
URL: https://gogs.example.com/owner/myproject/issues/42

Description:
Need OAuth2 support for user authentication. Should integrate with existing
user management system.

Comments (2):
────────────────────────────────────────────────────────────
[Opus/Planning] 2024-01-15 15:45:00
Breaking this into subtasks:
1. OAuth2 flow implementation
2. Token storage and validation
3. User session management

────────────────────────────────────────────────────────────
[Sonnet/Backend] 2024-01-16 09:15:00
Claiming task 1 (OAuth2 flow). Starting implementation.
────────────────────────────────────────────────────────────
```

**Output Format (JSON):**
```json
{
  "number": 42,
  "title": "Implement user authentication",
  "state": "open",
  "body": "Need OAuth2 support...",
  "labels": ["feature", "needs-planning"],
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-16T14:22:00Z",
  "creator": "alice",
  "comments": [
    {
      "id": 123,
      "body": "[Opus/Planning] Breaking this into subtasks...",
      "created_at": "2024-01-15T15:45:00Z",
      "user": "bot-opus"
    }
  ],
  "url": "https://gogs.example.com/owner/myproject/issues/42"
}
```

**Exit Codes:**
- 0: Success
- 1: Configuration or API error
- 2: Issue not found

---

### `gog issue create`

Create a new issue.

**Usage:**
```bash
gog issue create <title> [OPTIONS]
```

**Options:**
```
--repo <owner/repo>   Repository (required if no default in config)
--body <text>         Issue body/description (alternative: read from stdin)
--label <label>       Add label (can be repeated)
```

**Examples:**

Simple issue:
```bash
gog issue create "Fix login bug" --repo owner/myproject --body "Users can't log in"
```

From stdin:
```bash
cat issue_description.md | gog issue create "Feature: Export data" --repo owner/myproject
```

With labels:
```bash
gog issue create "Database migration needed" --repo owner/myproject --label bug --label high
```

**Signature Injection:**
The profile signature is automatically prepended to the body:
```
Input body: "Users can't log in"
Stored body: "[Human] Users can't log in"
```

**Output (Human):**
```
Created issue #43: Fix login bug
URL: https://gogs.example.com/owner/myproject/issues/43
```

**Output (JSON):**
```json
{
  "number": 43,
  "title": "Fix login bug",
  "state": "open",
  "url": "https://gogs.example.com/owner/myproject/issues/43"
}
```

**Exit Codes:**
- 0: Success
- 1: Configuration, validation, or API error

---

### `gog issue comment`

Add a comment to an existing issue.

**Usage:**
```bash
gog issue comment <number> <text> [OPTIONS]
```

**Options:**
```
--repo <owner/repo>   Repository (required if no default in config)
```

**Examples:**

Single-line comment:
```bash
gog issue comment 42 "Working on this now" --repo owner/myproject
```

Multi-line comment (shell-dependent):
```bash
gog issue comment 42 "Implementation complete.
Tests passing.
Ready for review." --repo owner/myproject
```

**Signature Injection:**
The profile signature is automatically prepended:
```
Input: "Working on this now"
Stored: "[Sonnet/Backend] Working on this now"
```

**Output (Human):**
```
Added comment to issue #42
```

**Output (JSON):**
```json
{
  "id": 124,
  "body": "[Sonnet/Backend] Working on this now",
  "created_at": "2024-01-16T16:30:00Z"
}
```

**Exit Codes:**
- 0: Success
- 1: Configuration or API error
- 2: Issue not found

---

### `gog issue close`

Close an issue.

**Usage:**
```bash
gog issue close <number> [OPTIONS]
```

**Options:**
```
--repo <owner/repo>   Repository (required if no default in config)
```

**Example:**
```bash
gog issue close 42 --repo owner/myproject
```

**Output (Human):**
```
Closed issue #42
```

**Output (JSON):**
```json
{
  "number": 42,
  "state": "closed"
}
```

**Exit Codes:**
- 0: Success
- 1: Configuration or API error
- 2: Issue not found

---

### `gog issue reopen`

Reopen a closed issue.

**Usage:**
```bash
gog issue reopen <number> [OPTIONS]
```

**Options:**
```
--repo <owner/repo>   Repository (required if no default in config)
```

**Example:**
```bash
gog issue reopen 42 --repo owner/myproject
```

**Output (Human):**
```
Reopened issue #42
```

**Exit Codes:**
- 0: Success
- 1: Configuration or API error
- 2: Issue not found

---

### `gog issue label`

Add a label to an issue.

**Usage:**
```bash
gog issue label <number> <label> [OPTIONS]
```

**Options:**
```
--repo <owner/repo>   Repository (required if no default in config)
```

**Example:**
```bash
gog issue label 42 needs-review --repo owner/myproject
```

**Output (Human):**
```
Added label 'needs-review' to issue #42
```

**Exit Codes:**
- 0: Success
- 1: Configuration or API error
- 2: Issue not found

---

### `gog issue unlabel`

Remove a label from an issue.

**Usage:**
```bash
gog issue unlabel <number> <label> [OPTIONS]
```

**Options:**
```
--repo <owner/repo>   Repository (required if no default in config)
```

**Example:**
```bash
gog issue unlabel 42 agent-working --repo owner/myproject
```

**Output (Human):**
```
Removed label 'agent-working' from issue #42
```

**Exit Codes:**
- 0: Success
- 1: Configuration or API error
- 2: Issue not found

---

### `gog repo list`

List all repositories accessible to the user.

**Usage:**
```bash
gog repo list
```

**Output Format (Human):**
```
owner/repo1 - Main project repository
owner/repo2 - Testing utilities
owner/repo3 - Documentation

Total: 3 repositories
```

**Output Format (JSON):**
```json
[
  {
    "owner": "owner",
    "name": "repo1",
    "full_name": "owner/repo1",
    "description": "Main project repository",
    "private": false,
    "url": "https://gogs.example.com/owner/repo1"
  }
]
```

**Exit Codes:**
- 0: Success
- 1: Configuration or API error

---

## Environment Variables

```
GOGS_CONFIG      Override config file location
GOGS_PROFILE     Override default profile
```

**Example:**
```bash
export GOGS_CONFIG=/opt/gogs-config/custom.toml
export GOGS_PROFILE=opus-planning
gog issue list --all
```

---

## Common Workflows

### Human: Create and assign work
```bash
# Create issue
gog issue create "Implement export feature" --repo owner/project --body "CSV and JSON formats"

# Label for planning agent
gog issue label 50 needs-planning --repo owner/project
```

### Planning Agent: Break down work
```bash
# Find work
gog issue list --label needs-planning --json --repo owner/project

# Add detailed plan
gog --profile opus-planning issue comment 50 "Breaking into subtasks:
1. CSV export handler
2. JSON export handler  
3. UI integration
4. Testing" --repo owner/project

# Update labels
gog --profile opus-planning issue label 50 planned --repo owner/project
gog --profile opus-planning issue unlabel 50 needs-planning --repo owner/project
```

### Implementation Agent: Claim and work
```bash
# Find available work
gog --profile sonnet-backend issue list --label planned --repo owner/project --json

# Claim task
gog --profile sonnet-backend issue comment 50 "Claiming this task" --repo owner/project
gog --profile sonnet-backend issue label 50 agent-working --repo owner/project

# Report progress
gog --profile sonnet-backend issue comment 50 "CSV handler complete, tests passing" --repo owner/project

# Mark for review
gog --profile sonnet-backend issue label 50 needs-review --repo owner/project
gog --profile sonnet-backend issue unlabel 50 agent-working --repo owner/project
```

### Human: Review and close
```bash
# See what needs review
gog issue list --label needs-review --repo owner/project

# Review details
gog issue show 50 --repo owner/project

# Close if good
gog issue close 50 --repo owner/project
```

---

## Scripting Examples

### Find all issues worked on by planning agents
```bash
gog issue list --all --json | jq '.[] | select(.comments[]?.user | contains("opus"))'
```

### Count open issues per repo
```bash
gog issue list --all --open --json | jq 'group_by(.repo) | map({repo: .[0].repo, count: length})'
```

### Monitor specific issue for updates
```bash
while true; do
  gog issue show 42 --repo owner/project | grep "Updated:"
  sleep 60
done
```

### Batch label issues
```bash
for issue in 10 11 12 13; do
  gog issue label $issue high-priority --repo owner/project
done
```
