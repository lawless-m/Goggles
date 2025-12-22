use anyhow::{Context, Result};

use crate::api::GogsClient;
use crate::cli::IssueCommand;
use crate::config::{Config, Profile};
use crate::output::{
    format_created_comment, format_created_issue, format_issue_detail, format_issue_list,
    format_issue_updated, OutputFormat,
};

pub async fn handle(
    cmd: IssueCommand,
    client: &GogsClient,
    config: &Config,
    profile: &Profile,
    json: bool,
) -> Result<()> {
    let format = OutputFormat::from_json_flag(json);

    match cmd {
        IssueCommand::List {
            all,
            open,
            closed,
            repo,
            label,
        } => {
            let state = if closed {
                "closed"
            } else if open {
                "open"
            } else {
                "open" // default
            };

            if all {
                handle_list_all(client, state, &label, &format).await
            } else {
                let (owner, repo_name) = config.get_repo(repo.as_deref())?;
                handle_list_repo(client, &owner, &repo_name, state, &label, &format).await
            }
        }

        IssueCommand::Show { number, repo } => {
            let (owner, repo_name) = config.get_repo(repo.as_deref())?;
            handle_show(client, &owner, &repo_name, number, &format).await
        }

        IssueCommand::Create {
            title,
            repo,
            body,
            label,
        } => {
            let (owner, repo_name) = config.get_repo(repo.as_deref())?;
            handle_create(client, &owner, &repo_name, &title, body.as_deref(), label, profile, &format).await
        }

        IssueCommand::Comment { number, text, repo } => {
            let (owner, repo_name) = config.get_repo(repo.as_deref())?;
            handle_comment(client, &owner, &repo_name, number, &text, profile, &format).await
        }

        IssueCommand::Close { number, repo } => {
            let (owner, repo_name) = config.get_repo(repo.as_deref())?;
            handle_state_change(client, &owner, &repo_name, number, "closed", &format).await
        }

        IssueCommand::Reopen { number, repo } => {
            let (owner, repo_name) = config.get_repo(repo.as_deref())?;
            handle_state_change(client, &owner, &repo_name, number, "open", &format).await
        }

        IssueCommand::Label { number, label, repo } => {
            let (owner, repo_name) = config.get_repo(repo.as_deref())?;
            handle_add_label(client, &owner, &repo_name, number, &label, &format).await
        }

        IssueCommand::Unlabel { number, label, repo } => {
            let (owner, repo_name) = config.get_repo(repo.as_deref())?;
            handle_remove_label(client, &owner, &repo_name, number, &label, &format).await
        }
    }
}

async fn handle_list_all(
    client: &GogsClient,
    state: &str,
    labels: &[String],
    format: &OutputFormat,
) -> Result<()> {
    let repos = client.list_user_repos().await?;

    // Spawn parallel tasks for each repo
    let state = state.to_string();
    let handles: Vec<_> = repos
        .into_iter()
        .map(|repo| {
            let client = client.clone();
            let state = state.clone();
            let full_name = repo.full_name.clone();

            tokio::spawn(async move {
                let result = client
                    .list_issues(&repo.owner.username, &repo.name, &state)
                    .await;
                (full_name, result)
            })
        })
        .collect();

    // Collect results
    let mut all_issues = Vec::new();
    for handle in handles {
        match handle.await {
            Ok((repo_name, Ok(mut issues))) => {
                // Filter by labels if specified
                if !labels.is_empty() {
                    issues.retain(|issue| {
                        labels.iter().any(|label| {
                            issue.labels.iter().any(|l| l.name.eq_ignore_ascii_case(label))
                        })
                    });
                }
                all_issues.push((repo_name, issues));
            }
            Ok((repo_name, Err(e))) => {
                eprintln!("Warning: Failed to list issues for {}: {}", repo_name, e);
            }
            Err(e) => {
                eprintln!("Warning: Task failed: {}", e);
            }
        }
    }

    // Sort by repo name for consistent output
    all_issues.sort_by(|a, b| a.0.cmp(&b.0));

    let output = format_issue_list(all_issues, format);
    print!("{}", output);
    Ok(())
}

async fn handle_list_repo(
    client: &GogsClient,
    owner: &str,
    repo: &str,
    state: &str,
    labels: &[String],
    format: &OutputFormat,
) -> Result<()> {
    let mut issues = client.list_issues(owner, repo, state).await?;

    // Filter by labels if specified
    if !labels.is_empty() {
        issues.retain(|issue| {
            labels.iter().any(|label| {
                issue.labels.iter().any(|l| l.name.eq_ignore_ascii_case(label))
            })
        });
    }

    let repo_name = format!("{}/{}", owner, repo);
    let output = format_issue_list(vec![(repo_name, issues)], format);
    print!("{}", output);
    Ok(())
}

async fn handle_show(
    client: &GogsClient,
    owner: &str,
    repo: &str,
    number: i64,
    format: &OutputFormat,
) -> Result<()> {
    let issue = client.get_issue(owner, repo, number).await?;
    let comments = client.list_comments(owner, repo, number).await?;

    let output = format_issue_detail(&issue, &comments, format);
    print!("{}", output);
    Ok(())
}

async fn handle_create(
    client: &GogsClient,
    owner: &str,
    repo: &str,
    title: &str,
    body: Option<&str>,
    labels: Vec<String>,
    profile: &Profile,
    format: &OutputFormat,
) -> Result<()> {
    // Prepend signature to body
    let body_with_sig = match body {
        Some(b) => format!("{} {}", profile.signature, b),
        None => profile.signature.clone(),
    };

    let issue = client
        .create_issue(owner, repo, title, Some(&body_with_sig), labels)
        .await?;

    let output = format_created_issue(&issue, format);
    print!("{}", output);
    Ok(())
}

async fn handle_comment(
    client: &GogsClient,
    owner: &str,
    repo: &str,
    number: i64,
    text: &str,
    profile: &Profile,
    format: &OutputFormat,
) -> Result<()> {
    // Prepend signature to comment
    let comment_with_sig = format!("{} {}", profile.signature, text);

    let comment = client
        .create_comment(owner, repo, number, &comment_with_sig)
        .await?;

    let output = format_created_comment(&comment, format);
    print!("{}", output);
    Ok(())
}

async fn handle_state_change(
    client: &GogsClient,
    owner: &str,
    repo: &str,
    number: i64,
    state: &str,
    format: &OutputFormat,
) -> Result<()> {
    let issue = client.update_issue(owner, repo, number, Some(state)).await?;
    let action = if state == "closed" { "closed" } else { "reopened" };
    let output = format_issue_updated(&issue, action, format);
    print!("{}", output);
    Ok(())
}

async fn handle_add_label(
    client: &GogsClient,
    owner: &str,
    repo: &str,
    number: i64,
    label_name: &str,
    format: &OutputFormat,
) -> Result<()> {
    // Get all labels from repo to find the label ID
    let repo_labels = client.list_repo_labels(owner, repo).await?;
    let label = repo_labels
        .iter()
        .find(|l| l.name.eq_ignore_ascii_case(label_name))
        .context(format!("Label '{}' not found in repository", label_name))?;

    let _labels = client.add_labels_to_issue(owner, repo, number, vec![label.id]).await?;

    match format {
        OutputFormat::Human => {
            println!("Label '{}' added to issue #{}", label_name, number);
        }
        OutputFormat::Json => {
            println!(r#"{{"status": "success", "label": "{}", "issue": {}}}"#, label_name, number);
        }
    }
    Ok(())
}

async fn handle_remove_label(
    client: &GogsClient,
    owner: &str,
    repo: &str,
    number: i64,
    label_name: &str,
    format: &OutputFormat,
) -> Result<()> {
    // Get all labels from repo to find the label ID
    let repo_labels = client.list_repo_labels(owner, repo).await?;
    let label = repo_labels
        .iter()
        .find(|l| l.name.eq_ignore_ascii_case(label_name))
        .context(format!("Label '{}' not found in repository", label_name))?;

    client.remove_label_from_issue(owner, repo, number, label.id).await?;

    match format {
        OutputFormat::Human => {
            println!("Label '{}' removed from issue #{}", label_name, number);
        }
        OutputFormat::Json => {
            println!(r#"{{"status": "success", "label": "{}", "issue": {}}}"#, label_name, number);
        }
    }
    Ok(())
}
