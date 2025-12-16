use crate::api::types::{Comment, Issue, Repository};

pub enum OutputFormat {
    Human,
    Json,
}

impl OutputFormat {
    pub fn from_json_flag(json: bool) -> Self {
        if json {
            OutputFormat::Json
        } else {
            OutputFormat::Human
        }
    }
}

pub fn format_issue_list(issues: Vec<(String, Vec<Issue>)>, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Human => format_issues_human(issues),
        OutputFormat::Json => format_issues_json(issues),
    }
}

fn format_issues_human(issues: Vec<(String, Vec<Issue>)>) -> String {
    let mut output = String::new();
    let mut total = 0;
    let mut repo_count = 0;

    for (repo, repo_issues) in &issues {
        if !repo_issues.is_empty() {
            repo_count += 1;
            output.push_str(&format!("\n{}\n", repo));

            for issue in repo_issues {
                let labels: Vec<String> = issue
                    .labels
                    .iter()
                    .map(|l| format!("[{}]", l.name))
                    .collect();
                let labels_str = if labels.is_empty() {
                    String::new()
                } else {
                    format!(" {}", labels.join(" "))
                };

                output.push_str(&format!(
                    "  #{:<4} [{}]{} {}\n",
                    issue.number, issue.state, labels_str, issue.title
                ));
                total += 1;
            }
        }
    }

    if total == 0 {
        output.push_str("\nNo issues found.\n");
    } else {
        output.push_str(&format!(
            "\nTotal: {} issue(s) across {} repo(s)\n",
            total, repo_count
        ));
    }

    output
}

fn format_issues_json(issues: Vec<(String, Vec<Issue>)>) -> String {
    #[derive(serde::Serialize)]
    struct IssueWithRepo {
        repo: String,
        #[serde(flatten)]
        issue: Issue,
    }

    let flattened: Vec<IssueWithRepo> = issues
        .into_iter()
        .flat_map(|(repo, repo_issues)| {
            repo_issues.into_iter().map(move |issue| IssueWithRepo {
                repo: repo.clone(),
                issue,
            })
        })
        .collect();

    serde_json::to_string_pretty(&flattened).unwrap_or_else(|_| "[]".to_string())
}

pub fn format_issue_detail(issue: &Issue, comments: &[Comment], format: &OutputFormat) -> String {
    match format {
        OutputFormat::Human => format_issue_detail_human(issue, comments),
        OutputFormat::Json => format_issue_detail_json(issue, comments),
    }
}

fn format_issue_detail_human(issue: &Issue, comments: &[Comment]) -> String {
    let mut output = String::new();

    output.push_str(&format!("#{} {}\n", issue.number, issue.title));
    output.push_str(&format!("State: {}\n", issue.state));
    output.push_str(&format!("Author: {}\n", issue.user.username));
    output.push_str(&format!("Created: {}\n", issue.created_at));
    output.push_str(&format!("Updated: {}\n", issue.updated_at));

    if !issue.labels.is_empty() {
        let labels: Vec<&str> = issue.labels.iter().map(|l| l.name.as_str()).collect();
        output.push_str(&format!("Labels: {}\n", labels.join(", ")));
    }

    output.push_str(&format!("URL: {}\n", issue.html_url));

    if let Some(body) = &issue.body {
        if !body.is_empty() {
            output.push_str(&format!("\n{}\n", body));
        }
    }

    if !comments.is_empty() {
        output.push_str(&format!("\n--- {} comment(s) ---\n", comments.len()));
        for comment in comments {
            output.push_str(&format!(
                "\n@{} ({})\n{}\n",
                comment.user.username, comment.created_at, comment.body
            ));
        }
    }

    output
}

fn format_issue_detail_json(issue: &Issue, comments: &[Comment]) -> String {
    #[derive(serde::Serialize)]
    struct IssueDetail<'a> {
        #[serde(flatten)]
        issue: &'a Issue,
        comment_list: &'a [Comment],
    }

    let detail = IssueDetail {
        issue,
        comment_list: comments,
    };

    serde_json::to_string_pretty(&detail).unwrap_or_else(|_| "{}".to_string())
}

pub fn format_repo_list(repos: &[Repository], format: &OutputFormat) -> String {
    match format {
        OutputFormat::Human => format_repos_human(repos),
        OutputFormat::Json => format_repos_json(repos),
    }
}

fn format_repos_human(repos: &[Repository]) -> String {
    let mut output = String::new();

    if repos.is_empty() {
        output.push_str("No repositories found.\n");
        return output;
    }

    output.push_str(&format!("Found {} repository(ies):\n\n", repos.len()));

    for repo in repos {
        let visibility = if repo.private { "[private]" } else { "[public]" };
        output.push_str(&format!("  {} {}\n", repo.full_name, visibility));
        if let Some(desc) = &repo.description {
            if !desc.is_empty() {
                output.push_str(&format!("    {}\n", desc));
            }
        }
    }

    output
}

fn format_repos_json(repos: &[Repository]) -> String {
    serde_json::to_string_pretty(repos).unwrap_or_else(|_| "[]".to_string())
}

pub fn format_created_issue(issue: &Issue, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Human => format!("Created issue #{}: {}\nURL: {}\n", issue.number, issue.title, issue.html_url),
        OutputFormat::Json => serde_json::to_string_pretty(issue).unwrap_or_else(|_| "{}".to_string()),
    }
}

pub fn format_created_comment(comment: &Comment, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Human => format!("Comment added by @{} at {}\n", comment.user.username, comment.created_at),
        OutputFormat::Json => serde_json::to_string_pretty(comment).unwrap_or_else(|_| "{}".to_string()),
    }
}

pub fn format_issue_updated(issue: &Issue, action: &str, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Human => format!("Issue #{} {}: {}\n", issue.number, action, issue.title),
        OutputFormat::Json => serde_json::to_string_pretty(issue).unwrap_or_else(|_| "{}".to_string()),
    }
}
