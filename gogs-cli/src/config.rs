use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Defaults {
    pub repo: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub gogs_user: String,
    pub token: String,
    pub role: String,
    pub signature: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        let contents = fs::read_to_string(&path)
            .context(format!("Failed to read config from {:?}. Run 'gog init' to create configuration.", path))?;

        let config: Config = toml::from_str(&contents)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        fs::write(&path, &contents)?;

        // Set restrictive permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        if let Ok(path) = std::env::var("GOGS_CONFIG") {
            return Ok(PathBuf::from(path));
        }

        let config_dir = dirs::config_dir()
            .context("Failed to determine config directory")?;

        Ok(config_dir.join("gogs-cli").join("config.toml"))
    }

    pub fn get_profile(&self, name: Option<&str>) -> Result<&Profile> {
        let profile_name = name
            .or(self.defaults.profile.as_deref())
            .unwrap_or("default");

        self.profiles.get(profile_name)
            .context(format!("Profile '{}' not found in config", profile_name))
    }

    pub fn get_repo(&self, repo: Option<&str>) -> Result<(String, String)> {
        let repo_str = repo
            .or(self.defaults.repo.as_deref())
            .context("Repository not specified. Use --repo owner/name or set defaults.repo in config")?;

        parse_repo(repo_str)
    }
}

pub fn parse_repo(repo: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid repository format. Expected 'owner/repo', got '{}'", repo);
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                url: "https://gogs.example.com".to_string(),
            },
            defaults: Defaults::default(),
            profiles: HashMap::new(),
        }
    }
}
