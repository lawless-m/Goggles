use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io::{self, Write};

use crate::api::GogsClient;
use crate::config::{Config, Defaults, Profile, ServerConfig};

pub async fn handle_init() -> Result<()> {
    println!("Gogs CLI Configuration Setup");
    println!("=============================\n");

    // Check if config already exists
    let config_path = Config::config_path()?;
    if config_path.exists() {
        println!("Config file already exists at {:?}", config_path);
        print!("Overwrite? [y/N]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    // Get server URL
    print!("Gogs server URL (e.g., https://gogs.example.com): ");
    io::stdout().flush()?;
    let mut server_url = String::new();
    io::stdin().read_line(&mut server_url)?;
    let server_url = server_url.trim().to_string();

    if server_url.is_empty() {
        anyhow::bail!("Server URL cannot be empty");
    }

    // Get profile name
    print!("Profile name [default]: ");
    io::stdout().flush()?;
    let mut profile_name = String::new();
    io::stdin().read_line(&mut profile_name)?;
    let profile_name = profile_name.trim();
    let profile_name = if profile_name.is_empty() {
        "default".to_string()
    } else {
        profile_name.to_string()
    };

    // Get Gogs username
    print!("Gogs username: ");
    io::stdout().flush()?;
    let mut gogs_user = String::new();
    io::stdin().read_line(&mut gogs_user)?;
    let gogs_user = gogs_user.trim().to_string();

    if gogs_user.is_empty() {
        anyhow::bail!("Username cannot be empty");
    }

    // Get API token
    print!("API token (from Gogs settings): ");
    io::stdout().flush()?;
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    let token = token.trim().to_string();

    if token.is_empty() {
        anyhow::bail!("API token cannot be empty");
    }

    // Get role description
    print!("Role description (e.g., 'Human Developer' or 'Planning Agent') [Human]: ");
    io::stdout().flush()?;
    let mut role = String::new();
    io::stdin().read_line(&mut role)?;
    let role = role.trim();
    let role = if role.is_empty() {
        "Human".to_string()
    } else {
        role.to_string()
    };

    // Get signature
    let default_sig = format!("[{}]", role);
    print!("Comment signature [{}]: ", default_sig);
    io::stdout().flush()?;
    let mut signature = String::new();
    io::stdin().read_line(&mut signature)?;
    let signature = signature.trim();
    let signature = if signature.is_empty() {
        default_sig
    } else {
        signature.to_string()
    };

    // Test connection
    println!("\nTesting connection to {}...", server_url);
    let client = GogsClient::new(server_url.clone(), token.clone());

    match client.list_user_repos().await {
        Ok(repos) => {
            println!("Connection successful! Found {} accessible repositories.", repos.len());
        }
        Err(e) => {
            println!("Warning: Connection test failed: {}", e);
            print!("Save config anyway? [y/N]: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Aborted.");
                return Ok(());
            }
        }
    }

    // Get default repo (optional)
    print!("Default repository (owner/repo, optional): ");
    io::stdout().flush()?;
    let mut default_repo = String::new();
    io::stdin().read_line(&mut default_repo)?;
    let default_repo = default_repo.trim();
    let default_repo = if default_repo.is_empty() {
        None
    } else {
        // Validate format
        if !default_repo.contains('/') {
            println!("Warning: Invalid repo format. Should be 'owner/repo'. Skipping default.");
            None
        } else {
            Some(default_repo.to_string())
        }
    };

    // Create config
    let profile = Profile {
        gogs_user,
        token,
        role,
        signature,
    };

    let mut profiles = HashMap::new();
    profiles.insert(profile_name.clone(), profile);

    let config = Config {
        server: ServerConfig { url: server_url },
        defaults: Defaults {
            repo: default_repo,
            profile: Some(profile_name.clone()),
        },
        profiles,
    };

    // Save config
    config.save().context("Failed to save configuration")?;

    println!("\nConfiguration saved to {:?}", config_path);
    println!("Profile '{}' created.", profile_name);
    println!("\nYou can now use gog commands. Try:");
    println!("  gog repo list");
    println!("  gog issue list --all");

    Ok(())
}
