use anyhow::Result;
use clap::Parser;
use std::process::ExitCode;

mod api;
mod cli;
mod commands;
mod config;
mod error;
mod output;

use cli::Cli;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> Result<()> {
    commands::dispatch(cli).await
}
