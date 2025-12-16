use super::client::GogsClient;
use super::types::Repository;
use anyhow::Result;

impl GogsClient {
    pub async fn list_user_repos(&self) -> Result<Vec<Repository>> {
        let path = "/user/repos";
        let resp = self.get(path).await?;
        let repos: Vec<Repository> = resp.json().await?;
        Ok(repos)
    }

    #[allow(dead_code)]
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository> {
        let path = format!("/repos/{}/{}", owner, repo);
        let resp = self.get(&path).await?;
        let repository: Repository = resp.json().await?;
        Ok(repository)
    }
}
