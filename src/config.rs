use std::collections::HashSet;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::engine::Manager;

/// Homebrew-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BrewConfig {
    /// Pass --greedy to brew upgrade (include auto-updating casks)
    #[serde(default)]
    pub greedy: bool,
}

/// Mise-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiseConfig {
    /// Pass --yes to mise self-update (skip confirmation prompts)
    #[serde(default = "default_true")]
    pub yes: bool,
}

fn default_true() -> bool {
    true
}

impl Default for MiseConfig {
    fn default() -> Self {
        Self { yes: true }
    }
}

/// User configuration for yup
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Enabled managers (stored as lowercase strings)
    pub enabled_managers: Vec<Manager>,
    /// Homebrew-specific settings
    #[serde(default)]
    pub brew: BrewConfig,
    /// Mise-specific settings
    #[serde(default)]
    pub mise: MiseConfig,
}

impl Config {
    /// Get the config file path (~/.config/yup/config.toml)
    pub fn path() -> Option<PathBuf> {
        let proj_dirs = ProjectDirs::from("", "", "yup")?;
        Some(proj_dirs.config_dir().join("config.toml"))
    }

    /// Load config from disk, returns None if not exists or invalid
    pub fn load() -> Option<Self> {
        let path = Self::path()?;
        let content = std::fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    /// Save config to disk
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::path().ok_or_else(|| anyhow::anyhow!("Cannot determine config path"))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// Check if config file exists
    pub fn exists() -> bool {
        Self::path().map(|p| p.exists()).unwrap_or(false)
    }

    /// Get enabled managers as a HashSet
    pub fn enabled_manager_set(&self) -> HashSet<Manager> {
        self.enabled_managers.iter().copied().collect()
    }
}
