# Gogs API Reference

## Overview

Gogs provides a RESTful API compatible with GitHub API v3. This document covers the endpoints needed for the gogs-cli tool.

**Base URL:** `https://<gogs-server>/api/v1`

**Authentication:** 
```
Authorization: token <API_TOKEN>
```

All authenticated requests must include this header.

## API Endpoints Used

### Authentication Test

**Endpoint:** `GET /user`

**Purpose:** Verify token is valid and get user information

**Request:**
```
GET /api/v1/user
Authorization: token abc123...
```

**Response (200 OK):**
```json
{
  "id": 1,
  "username": "myuser",
  "email": "user@example.com",
  "full_name": "My User"
}
```

**Error (401 Unauthorized):**
```json
{
  "message": "access token is not exist"
}
```

---

### List User Repositories

**Endpoint:** `GET /user/repos`

**Purpose:** Get all repositories accessible to the authenticated user

**Request:**
```
GET /api/v1/user/repos
Authorization: token abc123...
```

**Query Parameters:**
- None required for basic list

**Response (200 OK):**
```json
[
  {
    "id": 10,
    "owner": {
      "id": 1,
      "username": "owner",
      "full_name": "Owner Name"
    },
    "name": "repo1",
    "full_name": "owner/repo1",
    "description": "Main project repository",
    "private": false,
    "fork": false,
    "html_url": "https://gogs.example.com/owner/repo1",
    "clone_url": "https://gogs.example.com/owner/repo1.git"
  }
]
```

---

### List Repository Issues

**Endpoint:** `GET /repos/:owner/:repo/issues`

**Purpose:** Get issues for a specific repository

**Request:**
```
GET /api/v1/repos/owner/myrepo/issues?state=open
Authorization: token abc123...
```

**Query Parameters:**
- `state` (optional): Filter by state. Values: `open`, `closed`, `all`. Default: `open`
- `labels` (optional): Comma-separated list of label names
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Issues per page (default: 10, max: 50)

**Response (200 OK):**
```json
[
  {
    "id": 100,
    "number": 42,
    "title": "Fix database connection",
    "body": "Connection timeout issues...",
    "user": {
      "id": 1,
      "username": "alice"
    },
    "labels": [
      {
        "id": 1,
        "name": "bug",
        "color": "fc2929"
      }
    ],
    "state": "open",
    "comments": 3,
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-16T14:22:00Z",
    "html_url": "https://gogs.example.com/owner/myrepo/issues/42"
  }
]
```

---

### Get Single Issue

**Endpoint:** `GET /repos/:owner/:repo/issues/:number`

**Purpose:** Get detailed information about a specific issue

**Request:**
```
GET /api/v1/repos/owner/myrepo/issues/42
Authorization: token abc123...
```

**Response (200 OK):**
```json
{
  "id": 100,
  "number": 42,
  "title": "Fix database connection",
  "body": "Connection timeout issues...",
  "user": {
    "id": 1,
    "username": "alice",
    "full_name": "Alice Smith"
  },
  "labels": [
    {
      "id": 1,
      "name": "bug",
      "color": "fc2929"
    }
  ],
  "state": "open",
  "comments": 3,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-16T14:22:00Z",
  "html_url": "https://gogs.example.com/owner/myrepo/issues/42"
}
```

**Error (404 Not Found):**
```json
{
  "message": "Not Found"
}
```

---

### Create Issue

**Endpoint:** `POST /repos/:owner/:repo/issues`

**Purpose:** Create a new issue

**Request:**
```
POST /api/v1/repos/owner/myrepo/issues
Authorization: token abc123...
Content-Type: application/json

{
  "title": "New feature request",
  "body": "Description of the feature...",
  "labels": [1, 2]
}
```

**Request Body:**
- `title` (required): Issue title
- `body` (optional): Issue description
- `assignee` (optional): Username to assign
- `milestone` (optional): Milestone ID
- `labels` (optional): Array of label IDs

**Response (201 Created):**
```json
{
  "id": 101,
  "number": 43,
  "title": "New feature request",
  "body": "Description of the feature...",
  "user": {
    "id": 1,
    "username": "myuser"
  },
  "labels": [],
  "state": "open",
  "created_at": "2024-01-16T10:00:00Z",
  "updated_at": "2024-01-16T10:00:00Z",
  "html_url": "https://gogs.example.com/owner/myrepo/issues/43"
}
```

---

### Edit Issue

**Endpoint:** `PATCH /repos/:owner/:repo/issues/:number`

**Purpose:** Update an existing issue (change state, title, body, etc.)

**Request:**
```
PATCH /api/v1/repos/owner/myrepo/issues/42
Authorization: token abc123...
Content-Type: application/json

{
  "state": "closed"
}
```

**Request Body (all optional):**
- `title`: New title
- `body`: New body
- `state`: `open` or `closed`
- `assignee`: Username to assign
- `milestone`: Milestone ID

**Response (201 Created):**
```json
{
  "id": 100,
  "number": 42,
  "title": "Fix database connection",
  "state": "closed",
  "updated_at": "2024-01-16T15:00:00Z"
}
```

---

### List Issue Comments

**Endpoint:** `GET /repos/:owner/:repo/issues/:number/comments`

**Purpose:** Get all comments for an issue

**Request:**
```
GET /api/v1/repos/owner/myrepo/issues/42/comments
Authorization: token abc123...
```

**Query Parameters:**
- `since` (optional): Only show comments updated after this time (ISO 8601 format)

**Response (200 OK):**
```json
[
  {
    "id": 200,
    "body": "[Opus/Planning] Breaking this into subtasks...",
    "user": {
      "id": 2,
      "username": "bot-opus",
      "full_name": "Opus Bot"
    },
    "created_at": "2024-01-15T15:45:00Z",
    "updated_at": "2024-01-15T15:45:00Z"
  },
  {
    "id": 201,
    "body": "[Sonnet/Backend] Starting implementation...",
    "user": {
      "id": 3,
      "username": "bot-sonnet"
    },
    "created_at": "2024-01-16T09:15:00Z",
    "updated_at": "2024-01-16T09:15:00Z"
  }
]
```

---

### Create Issue Comment

**Endpoint:** `POST /repos/:owner/:repo/issues/:number/comments`

**Purpose:** Add a comment to an issue

**Request:**
```
POST /api/v1/repos/owner/myrepo/issues/42/comments
Authorization: token abc123...
Content-Type: application/json

{
  "body": "[Haiku/Triage] Labeled as high priority"
}
```

**Request Body:**
- `body` (required): Comment text

**Response (201 Created):**
```json
{
  "id": 202,
  "body": "[Haiku/Triage] Labeled as high priority",
  "user": {
    "id": 4,
    "username": "bot-haiku"
  },
  "created_at": "2024-01-16T10:30:00Z",
  "updated_at": "2024-01-16T10:30:00Z"
}
```

---

### List Repository Labels

**Endpoint:** `GET /repos/:owner/:repo/labels`

**Purpose:** Get all labels defined for a repository

**Request:**
```
GET /api/v1/repos/owner/myrepo/labels
Authorization: token abc123...
```

**Response (200 OK):**
```json
[
  {
    "id": 1,
    "name": "bug",
    "color": "fc2929"
  },
  {
    "id": 2,
    "name": "feature",
    "color": "84b6eb"
  }
]
```

---

### Add Label to Issue

**Endpoint:** `POST /repos/:owner/:repo/issues/:number/labels`

**Purpose:** Add one or more labels to an issue

**Request:**
```
POST /api/v1/repos/owner/myrepo/issues/42/labels
Authorization: token abc123...
Content-Type: application/json

{
  "labels": [1, 2]
}
```

**Request Body:**
- `labels` (required): Array of label IDs

**Response (200 OK):**
```json
[
  {
    "id": 1,
    "name": "bug",
    "color": "fc2929"
  },
  {
    "id": 2,
    "name": "feature",
    "color": "84b6eb"
  }
]
```

**Note:** Gogs may also support adding labels by name instead of ID. Implementation should check and use the simpler method.

---

### Remove Label from Issue

**Endpoint:** `DELETE /repos/:owner/:repo/issues/:number/labels/:id`

**Purpose:** Remove a specific label from an issue

**Request:**
```
DELETE /api/v1/repos/owner/myrepo/issues/42/labels/1
Authorization: token abc123...
```

**Response (204 No Content):**
Empty body

---

### Replace Issue Labels

**Endpoint:** `PUT /repos/:owner/:repo/issues/:number/labels`

**Purpose:** Replace all labels on an issue

**Request:**
```
PUT /api/v1/repos/owner/myrepo/issues/42/labels
Authorization: token abc123...
Content-Type: application/json

{
  "labels": [2, 3]
}
```

**Request Body:**
- `labels` (required): Array of label IDs (replaces all existing)

**Response (200 OK):**
```json
[
  {
    "id": 2,
    "name": "feature",
    "color": "84b6eb"
  },
  {
    "id": 3,
    "name": "high-priority",
    "color": "e11d21"
  }
]
```

---

## Implementation Notes

### Label Management Strategy

For adding/removing single labels, we have two options:

**Option 1: Use individual endpoints**
- `POST /repos/:owner/:repo/issues/:number/labels` to add
- `DELETE /repos/:owner/:repo/issues/:number/labels/:id` to remove
- Requires knowing label IDs

**Option 2: Use replace endpoint**
1. Get current labels
2. Add/remove from list
3. `PUT /repos/:owner/:repo/issues/:number/labels` with new list

Recommend **Option 1** for simplicity if label IDs can be cached or looked up efficiently. Otherwise **Option 2** for name-based operations.

### Pagination

Most list endpoints support pagination:
```
?page=1&per_page=50
```

For initial implementation, fetching all results with multiple requests is acceptable. Can optimize later with:
- Streaming results
- Caching
- Smarter pagination

### Rate Limiting

Gogs typically doesn't have aggressive rate limiting like GitHub, but consider:
- Reasonable delays between bulk operations
- Exponential backoff on errors
- Connection pooling (reqwest handles this)

### Error Handling

Common HTTP status codes:
- `200 OK`: Success (GET)
- `201 Created`: Success (POST)
- `204 No Content`: Success (DELETE)
- `400 Bad Request`: Invalid input
- `401 Unauthorized`: Bad token
- `403 Forbidden`: No permission
- `404 Not Found`: Resource doesn't exist
- `422 Unprocessable Entity`: Validation error
- `500 Internal Server Error`: Server error

Always check status code and parse error message from response body.

### API Version Compatibility

Gogs API is relatively stable. Current endpoints work with Gogs 0.11.x and later. If deployment uses older version, may need to adjust.

Test compatibility with:
```bash
curl https://gogs.example.com/api/v1/version
```

### Testing Strategy

For development/testing, consider:
1. Mock API server using `wiremock` crate
2. Docker container with Gogs for integration tests
3. Manual testing against real instance

All endpoint responses in this document are based on Gogs API documentation and may vary slightly by version.

## API Client Implementation Pattern

```rust
pub struct GogsClient {
    base_url: String,
    token: String,
    client: reqwest::Client,
}

impl GogsClient {
    pub fn new(base_url: String, token: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { base_url, token, client }
    }
    
    async fn request(&self, method: Method, path: &str, body: Option<Value>) 
        -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut req = self.client
            .request(method, &url)
            .header("Authorization", format!("token {}", self.token))
            .header("Content-Type", "application/json");
        
        if let Some(body) = body {
            req = req.json(&body);
        }
        
        let resp = req.send().await?;
        
        if !resp.status().is_success() {
            return Err(GogsError::from_response(resp).await);
        }
        
        Ok(resp)
    }
}
```

## Quick Reference

| Operation | Method | Endpoint |
|-----------|--------|----------|
| Test auth | GET | `/user` |
| List repos | GET | `/user/repos` |
| List issues | GET | `/repos/:owner/:repo/issues` |
| Get issue | GET | `/repos/:owner/:repo/issues/:number` |
| Create issue | POST | `/repos/:owner/:repo/issues` |
| Update issue | PATCH | `/repos/:owner/:repo/issues/:number` |
| List comments | GET | `/repos/:owner/:repo/issues/:number/comments` |
| Add comment | POST | `/repos/:owner/:repo/issues/:number/comments` |
| List labels | GET | `/repos/:owner/:repo/labels` |
| Add label | POST | `/repos/:owner/:repo/issues/:number/labels` |
| Remove label | DELETE | `/repos/:owner/:repo/issues/:number/labels/:id` |
| Replace labels | PUT | `/repos/:owner/:repo/issues/:number/labels` |
