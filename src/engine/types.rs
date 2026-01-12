use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString};

/// Manager enum - represents each supported package or version manager.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    AsRefStr,
    Display,
    EnumString,
    EnumIter,
    EnumCount,
)]
#[strum(ascii_case_insensitive)]
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
    /// Set of managers that were found to be available.
    pub available_managers: HashSet<Manager>,
    /// Subset of available managers that have actionable implementations.
    pub actionable_managers: HashSet<Manager>,
}

impl Action {
    /// Create a new action
    #[must_use]
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
