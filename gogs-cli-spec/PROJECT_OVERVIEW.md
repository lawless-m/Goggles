# Gogs CLI - Multi-Agent Development Orchestration Tool

## What Is This?

A Rust-based command-line tool for interacting with Gogs issue trackers, designed specifically to coordinate multiple AI coding agents working across repositories.

## Why Build This?

### The Problem
- Multiple AI agents (Claude Opus, Sonnet, Haiku, local models like Qwen) have different strengths
- Need a way to coordinate their work across multiple repositories
- Want agents to work in isolated sandboxes safely
- Need clear audit trails of who did what
- Existing tools (like GitHub's `gh`) don't work with Gogs

### The Solution
A lightweight CLI tool that:
- Provides a consistent interface to Gogs issue tracking
- Supports multiple agent profiles (different identities and roles)
- Works cross-platform (Windows, Linux)
- Enables both human and programmatic use
- Requires no complex dependencies (single binary)

## Core Concept: Profile-Based Agent Orchestration

The tool uses **profiles** to represent different agent identities and roles:

```
haiku-triage     → Quick assessment and prioritization
haiku-webdev     → Frontend/UI implementation
opus-planning    → Architecture and task breakdown
opus-solving     → Complex problem solving
sonnet-backend   → Backend implementation
sonnet-database  → Database and schema work
qwen-local       → Local/offline tasks
```

Each profile has:
- Its own Gogs user identity (API token)
- A defined role and context
- A signature for comments (clear attribution)

## Typical Workflow

1. **Human creates issues** across various repositories as ideas arise
2. **Planning agent (opus-planning)** reviews issues, creates architecture plans, breaks down into subtasks
3. **Implementation agents (sonnet-backend, haiku-webdev)** claim subtasks and work in isolated sandboxes
4. **Agents report progress** via comments, ask questions, flag blockers
5. **Triage agent (haiku-triage)** helps prioritize and organize
6. **Human reviews** work and provides guidance
7. **Testing agents** validate in clean environments
8. **Issues close** when complete

## Key Benefits

### For Humans
- Dashboard view of all work across all repos (`gog issue list --all`)
- Clear visibility into what each agent is doing
- Ability to guide and redirect agents via comments
- Persistent record of decisions and rationale

### For Agents
- Clear task definitions (issues)
- Ability to communicate asynchronously (comments)
- Safe sandbox environments (can't break things)
- Defined roles and boundaries (via profiles)

### For Experimentation
- Try different models on different tasks
- Compare performance objectively
- Learn what works through real use
- Mix local and cloud agents as needed

## Technical Approach

- **Language:** Rust (single binary, no runtime, cross-platform)
- **Authentication:** Gogs API tokens (per profile)
- **Configuration:** TOML files with profile definitions
- **Output:** Human-readable by default, JSON for scripting
- **Distribution:** Single executable per platform

## Success Criteria

The tool succeeds if:
1. Agents can create, read, update, and close issues programmatically
2. Different profiles maintain clear identities in the issue tracker
3. Humans can easily see all work across all repositories
4. Works reliably on both Windows and Linux
5. Simple enough that new agents can be added easily

## What This Is NOT

- Not a git client (use git for that)
- Not a CI/CD system (use existing tools)
- Not an agent framework (agents bring their own intelligence)
- Not Gogs-specific features beyond issues (no PR handling, etc.)

This is pure coordination infrastructure.

## Next Steps

See DEVELOPMENT_PLAN.md for suggested implementation order and CONTENTS.md for guide to all documentation.
