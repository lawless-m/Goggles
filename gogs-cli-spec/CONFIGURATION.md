# Configuration Guide

## Configuration File Location

### Linux
```
~/.config/gogs-cli/config.toml
```

### Windows
```
%APPDATA%\gogs-cli\config.toml
```

### Environment Variable Override
```bash
export GOGS_CONFIG=/path/to/custom/config.toml
```

## Configuration File Format

The configuration uses TOML format with three main sections:

### Complete Example

```toml
[server]
url = "https://gogs.example.com"

[defaults]
repo = "owner/main-project"
profile = "default"

[profiles.default]
gogs_user = "alice"
token = "abc123def456ghi789"
role = "Human Developer"
signature = "[Human]"

[profiles.opus-planning]
gogs_user = "bot-opus"
token = "xyz789uvw456rst123"
role = "Architecture and Planning Agent"
signature = "[Opus/Planning]"

[profiles.opus-solving]
gogs_user = "bot-opus"
token = "xyz789uvw456rst123"
role = "Complex Problem Solving Agent"
signature = "[Opus/Solving]"

[profiles.sonnet-backend]
gogs_user = "bot-sonnet"
token = "mno345pqr678stu901"
role = "Backend Implementation Agent"
signature = "[Sonnet/Backend]"

[profiles.sonnet-database]
gogs_user = "bot-sonnet-db"
token = "aaa111bbb222ccc333"
role = "Database and Schema Agent"
signature = "[Sonnet/Database]"

[profiles.haiku-triage]
gogs_user = "bot-haiku"
token = "def012ghi345jkl678"
role = "Triage and Prioritization Agent"
signature = "[Haiku/Triage]"

[profiles.haiku-webdev]
gogs_user = "bot-haiku-web"
token = "bbb222ccc333ddd444"
role = "Frontend/Web Development Agent"
signature = "[Haiku/WebDev]"

[profiles.haiku-testing]
gogs_user = "bot-haiku-test"
token = "ccc333ddd444eee555"
role = "Testing and Verification Agent"
signature = "[Haiku/Testing]"

[profiles.qwen-local]
gogs_user = "bot-qwen"
token = "lmn567opq890rst234"
role = "Local Model Agent"
signature = "[Qwen/Local]"
```

## Section Details

### `[server]`

**Required fields:**
- `url`: Base URL of your Gogs instance (without trailing slash)

**Example:**
```toml
[server]
url = "https://gogs.company.com"
```

**Notes:**
- Include protocol (https:// or http://)
- Do not include `/api/v1` or trailing slash
- HTTPS is strongly recommended

### `[defaults]`

**Optional fields:**
- `repo`: Default repository in `owner/repo` format
- `profile`: Default profile name to use when `--profile` not specified

**Example:**
```toml
[defaults]
repo = "mycompany/main-project"
profile = "default"
```

**Notes:**
- If `repo` is set, commands don't require `--repo` flag
- If `profile` is not set, "default" is used
- Command-line flags override these defaults

### `[profiles.<name>]`

Each profile represents an agent identity and role.

**Required fields:**
- `gogs_user`: Gogs username for this profile
- `token`: API token for authentication
- `role`: Human-readable description of this agent's purpose
- `signature`: Text prepended to issues/comments

**Profile naming conventions:**
- Use lowercase with hyphens: `opus-planning`, `haiku-triage`
- Include model and role: `<model>-<role>`
- Keep names short but descriptive

**Signature conventions:**
```
[ModelName/Role]
```

Examples:
- `[Opus/Planning]`
- `[Sonnet/Backend]`
- `[Haiku/Triage]`
- `[Human]`

## Generating API Tokens

### Via Gogs Web UI

1. Log into Gogs as the bot user
2. Go to User Settings (top right menu)
3. Click "Applications" tab
4. In "Generate New Token" section:
   - Enter a name (e.g., "gogs-cli-opus-planning")
   - Click "Generate Token"
5. Copy the token immediately (only shown once)
6. Paste into config file

### Token Management

**Security best practices:**
- Create separate Gogs users for each agent profile
- Use descriptive token names
- Rotate tokens periodically
- Never commit config files with tokens to version control
- Set appropriate file permissions (0600 on Unix)

**Token permissions:**
Tokens have same permissions as the user. For agent users:
- Read access to relevant repositories
- Write access to create issues/comments
- No admin access needed

## Profile Selection Priority

When determining which profile to use, the tool checks in this order:

1. `--profile` command-line flag
2. `GOGS_PROFILE` environment variable
3. `defaults.profile` in config file
4. "default" (hardcoded fallback)

**Example:**
```bash
# Use opus-planning profile
gog --profile opus-planning issue list --all

# Use profile from environment
export GOGS_PROFILE=haiku-triage
gog issue list --all

# Use default profile from config
gog issue list --all
```

## Multiple Configuration Files

You can maintain separate config files for different contexts:

```bash
# Production config
gog issue list --all

# Staging config
GOGS_CONFIG=~/.config/gogs-cli/staging.toml gog issue list --all

# Local development
GOGS_CONFIG=./config-dev.toml gog issue list --all
```

## Sandbox/VM Configuration

For agent sandboxes, create minimal config files:

**sandbox-config.toml:**
```toml
[server]
url = "https://gogs.example.com"

[profiles.agent]
gogs_user = "bot-sonnet"
token = "agent-token-here"
role = "Sandbox Agent"
signature = "[Sandbox/Sonnet]"
```

Deploy to sandbox:
```bash
scp sandbox-config.toml vm:/tmp/gogs-config.toml
ssh vm
GOGS_CONFIG=/tmp/gogs-config.toml gog --profile agent issue list --all
```

## Configuration Validation

The tool validates configuration on load:

**Checked at load time:**
- File exists and is readable
- Valid TOML syntax
- Required fields present in `[server]`
- At least one profile defined

**Checked at runtime:**
- Profile exists when referenced
- Token is valid (on first API call)
- Repository format is valid (owner/repo)

**Common errors:**
```
Error: Profile 'opus-planning' not found in config
Error: Configuration error: missing field `token`
Error: Invalid repository format, expected 'owner/repo'
```

## File Permissions

### Unix/Linux
```bash
# Set secure permissions
chmod 600 ~/.config/gogs-cli/config.toml

# Verify
ls -l ~/.config/gogs-cli/config.toml
# Should show: -rw------- (owner read/write only)
```

**Warning:** If permissions are too open (world-readable), the tool should warn:
```
Warning: Config file permissions are too open (should be 0600)
```

### Windows
Windows uses ACLs. Ensure only your user account has access:
```powershell
# View current permissions
Get-Acl $env:APPDATA\gogs-cli\config.toml | Format-List

# Restrict to current user only (if needed)
$acl = Get-Acl $env:APPDATA\gogs-cli\config.toml
$acl.SetAccessRuleProtection($true, $false)
$rule = New-Object System.Security.AccessControl.FileSystemAccessRule(
    $env:USERNAME, "FullControl", "Allow"
)
$acl.AddAccessRule($rule)
Set-Acl $env:APPDATA\gogs-cli\config.toml $acl
```

## Example Configurations

### Minimal (Single User)
```toml
[server]
url = "https://gogs.example.com"

[profiles.default]
gogs_user = "alice"
token = "token123"
role = "Developer"
signature = "[Alice]"
```

### Multi-Agent (Full Setup)
```toml
[server]
url = "https://gogs.company.com"

[defaults]
repo = "company/main-project"
profile = "default"

[profiles.default]
gogs_user = "alice"
token = "human-token"
role = "Human Developer"
signature = "[Human]"

[profiles.opus-planning]
gogs_user = "bot-opus"
token = "opus-token"
role = "Planning Agent"
signature = "[Opus/Planning]"

[profiles.sonnet-backend]
gogs_user = "bot-sonnet"
token = "sonnet-token"
role = "Backend Agent"
signature = "[Sonnet/Backend]"

[profiles.haiku-triage]
gogs_user = "bot-haiku"
token = "haiku-token"
role = "Triage Agent"
signature = "[Haiku/Triage]"
```

### Experimentation (Multiple Roles per Model)
```toml
[server]
url = "https://gogs.example.com"

# Haiku with different roles
[profiles.haiku-triage]
gogs_user = "bot-haiku"
token = "haiku-token"
role = "Issue Triage"
signature = "[Haiku/Triage]"

[profiles.haiku-review]
gogs_user = "bot-haiku"
token = "haiku-token"
role = "Code Review"
signature = "[Haiku/Review]"

[profiles.haiku-testing]
gogs_user = "bot-haiku-test"
token = "haiku-test-token"
role = "Testing"
signature = "[Haiku/Testing]"

# Multiple local models
[profiles.qwen-local]
gogs_user = "bot-qwen"
token = "qwen-token"
role = "Local Agent"
signature = "[Qwen/Local]"

[profiles.llama-local]
gogs_user = "bot-llama"
token = "llama-token"
role = "Local Agent (Llama)"
signature = "[Llama/Local]"
```

## Troubleshooting

### Config file not found
```
Error: Failed to read config from "/home/user/.config/gogs-cli/config.toml"
```
**Solution:** Run `gog init` to create initial configuration.

### Invalid token
```
Error: Authentication failed: access token is not exist
```
**Solution:** 
1. Verify token in Gogs UI (Settings → Applications)
2. Generate new token if needed
3. Update config file

### Profile not found
```
Error: Profile 'opus-planning' not found in config
```
**Solution:** 
1. Check profile name spelling
2. Ensure profile is defined in config
3. Run `gog init` to add new profile

### Permission denied (Unix)
```
Error: Permission denied (os error 13)
```
**Solution:** Set correct permissions: `chmod 600 ~/.config/gogs-cli/config.toml`

## Migration and Updates

### Adding a new profile
1. Open config file in editor
2. Copy existing profile section
3. Rename and update fields
4. Generate new token in Gogs
5. Save and test

### Updating token
1. Generate new token in Gogs
2. Update `token` field in config
3. Save file
4. Test: `gog --profile <name> repo list`

### Changing server URL
1. Update `server.url` in config
2. Test connectivity: `gog repo list`
3. Update all profiles if moving to new server
