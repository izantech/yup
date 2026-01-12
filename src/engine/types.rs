use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Manager enum - each supported package manager/version manager
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)] // Platform-specific variants kept for cross-platform support
pub enum Manager {
    // System
    Brew,
    Port,
    Apt,
    Dnf,
    Pacman,
    Flatpak,
    Snap,
    Winget,
    Choco,
    Scoop,
    // Version managers
    Nvm,
    Fnm,
    Volta,
    Asdf,
    Mise,
    Pyenv,
    Rbenv,
    Rvm,
    Conda,
    // Language tools
    Npm,
    Pnpm,
    Yarn,
    Bun,
    Pip,
    Pipx,
    Poetry,
    Uv,
    Cargo,
    Rustup,
    Go,
    Gem,
    Composer,
    Helm,
    Krew,
    Sdkman,
    // Other
    Mas,
    SoftwareUpdate,
    // Fallback
    System,
    Unknown,
}

/// Tool detection result
#[derive(Debug, Clone)]
pub struct DetectedTool {
    #[allow(dead_code)] // Useful for future display
    pub name: String,
    pub path: PathBuf,
    pub manager: Manager,
}

/// What can be done
#[derive(Debug, Clone)]
#[allow(dead_code)] // Variants kept for future action filtering
pub enum ActionKind {
    Update,
    Upgrade,
    Cleanup,
    Check,
}

/// A single action to perform
#[derive(Debug, Clone)]
pub struct Action {
    pub manager: Manager,
    #[allow(dead_code)] // Useful for future action filtering by type
    pub kind: ActionKind,
    pub command: String,
    pub description: String,
    /// Whether this action requires root/admin privileges (sudo on Unix)
    pub requires_privilege: bool,
}

/// Result of scanning the system
#[derive(Debug, Default)]
pub struct ScanReport {
    pub detected_tools: Vec<DetectedTool>,
    pub available_managers: HashSet<Manager>,
    /// Managers that have PackageManager implementations AND return actions
    pub actionable_managers: HashSet<Manager>,
}

impl std::fmt::Display for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Manager {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "brew" => Ok(Manager::Brew),
            "port" => Ok(Manager::Port),
            "apt" => Ok(Manager::Apt),
            "dnf" => Ok(Manager::Dnf),
            "pacman" => Ok(Manager::Pacman),
            "flatpak" => Ok(Manager::Flatpak),
            "snap" => Ok(Manager::Snap),
            "winget" => Ok(Manager::Winget),
            "choco" => Ok(Manager::Choco),
            "scoop" => Ok(Manager::Scoop),
            "nvm" => Ok(Manager::Nvm),
            "fnm" => Ok(Manager::Fnm),
            "volta" => Ok(Manager::Volta),
            "asdf" => Ok(Manager::Asdf),
            "mise" => Ok(Manager::Mise),
            "pyenv" => Ok(Manager::Pyenv),
            "rbenv" => Ok(Manager::Rbenv),
            "rvm" => Ok(Manager::Rvm),
            "conda" => Ok(Manager::Conda),
            "npm" => Ok(Manager::Npm),
            "pnpm" => Ok(Manager::Pnpm),
            "yarn" => Ok(Manager::Yarn),
            "bun" => Ok(Manager::Bun),
            "pip" => Ok(Manager::Pip),
            "pipx" => Ok(Manager::Pipx),
            "poetry" => Ok(Manager::Poetry),
            "uv" => Ok(Manager::Uv),
            "cargo" => Ok(Manager::Cargo),
            "rustup" => Ok(Manager::Rustup),
            "go" => Ok(Manager::Go),
            "gem" => Ok(Manager::Gem),
            "composer" => Ok(Manager::Composer),
            "helm" => Ok(Manager::Helm),
            "krew" => Ok(Manager::Krew),
            "sdkman" => Ok(Manager::Sdkman),
            "mas" => Ok(Manager::Mas),
            "softwareupdate" => Ok(Manager::SoftwareUpdate),
            "system" => Ok(Manager::System),
            "unknown" => Ok(Manager::Unknown),
            _ => Err(format!("Unknown manager: {}", s)),
        }
    }
}
