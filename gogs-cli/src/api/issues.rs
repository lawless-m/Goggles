use super::client::GogsClient;
use super::types::{Comment, Issue, Label};
use anyhow::Result;
use serde_json::json;

impl GogsClient {
    pub async fn list_issues(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
    ) -> Result<Vec<Issue>> {
        let path = format!("/repos/{}/{}/issues?state={}", owner, repo, state);
        let resp = self.get(&path).await?;
        let issues: Vec<Issue> = resp.json().await?;
        Ok(issues)
    }

    pub async fn get_issue(&self, owner: &str, repo: &str, number: i64) -> Result<Issue> {
        let path = format!("/repos/{}/{}/issues/{}", owner, repo, number);
        let resp = self.get(&path).await?;
        let issue: Issue = resp.json().await?;
        Ok(issue)
    }

    pub async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: Option<&str>,
        labels: Vec<String>,
    ) -> Result<Issue> {
        let path = format!("/repos/{}/{}/issues", owner, repo);
        let mut payload = json!({
            "title": title,
        });

        if let Some(b) = body {
            payload["body"] = json!(b);
        }

        if !labels.is_empty() {
            payload["labels"] = json!(labels);
        }

        let resp = self.post(&path, payload).await?;
        let issue: Issue = resp.json().await?;
        Ok(issue)
    }

    pub async fn update_issue(
        &self,
        owner: &str,
        repo: &str,
        number: i64,
        state: Option<&str>,
    ) -> Result<Issue> {
        let path = format!("/repos/{}/{}/issues/{}", owner, repo, number);
        let mut payload = json!({});

        if let Some(s) = state {
            payload["state"] = json!(s);
        }

        let resp = self.patch(&path, payload).await?;
        let issue: Issue = resp.json().await?;
        Ok(issue)
    }

    pub async fn list_comments(
        &self,
        owner: &str,
        repo: &str,
        number: i64,
    ) -> Result<Vec<Comment>> {
        let path = format!("/repos/{}/{}/issues/{}/comments", owner, repo, number);
        let resp = self.get(&path).await?;
        let comments: Vec<Comment> = resp.json().await?;
        Ok(comments)
    }

    pub async fn create_comment(
        &self,
        owner: &str,
        repo: &str,
        number: i64,
        body: &str,
    ) -> Result<Comment> {
        let path = format!("/repos/{}/{}/issues/{}/comments", owner, repo, number);
        let payload = json!({ "body": body });
        let resp = self.post(&path, payload).await?;
        let comment: Comment = resp.json().await?;
        Ok(comment)
    }

    pub async fn list_repo_labels(&self, owner: &str, repo: &str) -> Result<Vec<Label>> {
        let path = format!("/repos/{}/{}/labels", owner, repo);
        let resp = self.get(&path).await?;
        let labels: Vec<Label> = resp.json().await?;
        Ok(labels)
    }

    pub async fn add_labels_to_issue(
        &self,
        owner: &str,
        repo: &str,
        number: i64,
        labels: Vec<i64>,
    ) -> Result<Vec<Label>> {
        let path = format!("/repos/{}/{}/issues/{}/labels", owner, repo, number);
        let payload = json!({ "labels": labels });
        let resp = self.post(&path, payload).await?;
        let result: Vec<Label> = resp.json().await?;
        Ok(result)
    }

    pub async fn remove_label_from_issue(
        &self,
        owner: &str,
        repo: &str,
        number: i64,
        label_id: i64,
    ) -> Result<()> {
        let path = format!("/repos/{}/{}/issues/{}/labels/{}", owner, repo, number, label_id);
        let _resp = self.request(reqwest::Method::DELETE, &path, None).await?;
        Ok(())
    }
}
