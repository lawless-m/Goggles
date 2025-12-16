use anyhow::Result;

use crate::api::GogsClient;
use crate::cli::RepoCommand;
use crate::output::{format_repo_list, OutputFormat};

pub async fn handle(cmd: RepoCommand, client: &GogsClient, json: bool) -> Result<()> {
    let format = OutputFormat::from_json_flag(json);

    match cmd {
        RepoCommand::List => handle_list(client, &format).await,
    }
}

async fn handle_list(client: &GogsClient, format: &OutputFormat) -> Result<()> {
    let repos = client.list_user_repos().await?;
    let output = format_repo_list(&repos, format);
    print!("{}", output);
    Ok(())
}
