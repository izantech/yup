use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Manager enum - each supported package manager/version manager
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)] // Platform-specific variants kept for cross-platform support
pub enum Manager {
    // System package managers
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
    Mas,
    SoftwareUpdate,
    // Version managers (with global upgrade support)
    Mise,
    Conda,
    // Language tools (with global upgrade support)
    Npm,
    Pnpm,
    Pipx,
    Cargo,
    Rustup,
    Gem,
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
            // System package managers
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
            "mas" => Ok(Manager::Mas),
            "softwareupdate" => Ok(Manager::SoftwareUpdate),
            // Version managers
            "mise" => Ok(Manager::Mise),
            "conda" => Ok(Manager::Conda),
            // Language tools
            "npm" => Ok(Manager::Npm),
            "pnpm" => Ok(Manager::Pnpm),
            "pipx" => Ok(Manager::Pipx),
            "cargo" => Ok(Manager::Cargo),
            "rustup" => Ok(Manager::Rustup),
            "gem" => Ok(Manager::Gem),
            // Fallback
            "system" => Ok(Manager::System),
            "unknown" => Ok(Manager::Unknown),
            _ => Err(format!("Unknown manager: {}", s)),
        }
    }
}
