use anyhow::Result;

use crate::api::GogsClient;
use crate::cli::{Cli, Commands};
use crate::config::Config;

pub mod init;
pub mod issue;
pub mod repo;

pub async fn dispatch(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init => init::handle_init().await,

        Commands::Issue(cmd) => {
            let config = Config::load()?;
            let profile = config.get_profile(cli.profile.as_deref())?;
            let client = GogsClient::new(config.server.url.clone(), profile.token.clone());

            issue::handle(cmd, &client, &config, profile, cli.json).await
        }

        Commands::Repo(cmd) => {
            let config = Config::load()?;
            let profile = config.get_profile(cli.profile.as_deref())?;
            let client = GogsClient::new(config.server.url.clone(), profile.token.clone());

            repo::handle(cmd, &client, cli.json).await
        }
    }
}
