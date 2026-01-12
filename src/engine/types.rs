use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Manager enum - represents each supported package or version manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

/// A tool detected on the system and its associated package manager.
#[derive(Debug, Clone)]
pub struct DetectedTool {
    /// Absolute path to the tool's executable.
    pub path: PathBuf,
    /// The manager identified as responsible for this tool.
    pub manager: Manager,
}

/// A single discrete action to be performed by a package manager.
#[derive(Debug, Clone)]
pub struct Action {
    /// The manager that will execute this action.
    pub manager: Manager,
    /// The shell command to execute.
    pub command: String,
    /// Human-readable description of what this action does.
    pub description: String,
    /// Whether this action requires root/admin privileges (sudo on Unix).
    pub requires_privilege: bool,
}

/// Comprehensive report of the system scan results.
#[derive(Debug, Default)]
pub struct ScanReport {
    /// List of all detected tools.
    pub detected_tools: Vec<DetectedTool>,
    /// Set of managers that were found to be available.
    pub available_managers: HashSet<Manager>,
    /// Subset of available managers that have actionable implementations.
    pub actionable_managers: HashSet<Manager>,
}

impl std::fmt::Display for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Manager {
    /// Canonical lowercase name for CLI filtering and config.
    pub fn as_str(&self) -> &'static str {
        match self {
            Manager::Brew => "brew",
            Manager::Port => "port",
            Manager::Apt => "apt",
            Manager::Dnf => "dnf",
            Manager::Pacman => "pacman",
            Manager::Flatpak => "flatpak",
            Manager::Snap => "snap",
            Manager::Winget => "winget",
            Manager::Choco => "choco",
            Manager::Scoop => "scoop",
            Manager::Mas => "mas",
            Manager::SoftwareUpdate => "softwareupdate",
            Manager::Mise => "mise",
            Manager::Conda => "conda",
            Manager::Npm => "npm",
            Manager::Pnpm => "pnpm",
            Manager::Pipx => "pipx",
            Manager::Cargo => "cargo",
            Manager::Rustup => "rustup",
            Manager::Gem => "gem",
            Manager::System => "system",
            Manager::Unknown => "unknown",
        }
    }

    /// Human-readable name for display.
    pub fn display_name(&self) -> &'static str {
        match self {
            Manager::Brew => "Brew",
            Manager::Port => "Port",
            Manager::Apt => "Apt",
            Manager::Dnf => "Dnf",
            Manager::Pacman => "Pacman",
            Manager::Flatpak => "Flatpak",
            Manager::Snap => "Snap",
            Manager::Winget => "Winget",
            Manager::Choco => "Choco",
            Manager::Scoop => "Scoop",
            Manager::Mas => "Mas",
            Manager::SoftwareUpdate => "SoftwareUpdate",
            Manager::Mise => "Mise",
            Manager::Conda => "Conda",
            Manager::Npm => "Npm",
            Manager::Pnpm => "Pnpm",
            Manager::Pipx => "Pipx",
            Manager::Cargo => "Cargo",
            Manager::Rustup => "Rustup",
            Manager::Gem => "Gem",
            Manager::System => "System",
            Manager::Unknown => "Unknown",
        }
    }
}

impl Action {
    /// Create a new action
    pub fn new(
        manager: Manager,
        command: impl Into<String>,
        description: impl Into<String>,
        requires_privilege: bool,
    ) -> Self {
        Self {
            manager,
            command: command.into(),
            description: description.into(),
            requires_privilege,
        }
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
