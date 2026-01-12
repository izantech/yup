//! Package manager implementations
//!
//! This module defines the `PackageManager` trait and implementations for
//! various system package managers.

use super::types::{Action, Manager};

/// Trait defining the interface for package manager implementations.
pub trait PackageManager {
    /// Returns a list of actions to update the package manager's metadata or index.
    /// Example: `brew update` or `apt update`.
    fn update_actions(&self) -> Vec<Action>;

    /// Returns a list of actions to upgrade the packages managed by this tool.
    /// Example: `brew upgrade` or `npm update -g`.
    fn upgrade_actions(&self) -> Vec<Action>;

    /// Returns a list of actions to check for outdated packages without performing an update.
    /// Example: `brew outdated`.
    fn check_actions(&self) -> Vec<Action> {
        vec![] // Default implementation returns no actions
    }
}

/// Check if a command exists in PATH
pub fn command_exists(cmd: &str) -> bool {
    which::which(cmd).is_ok()
}

// macOS managers
#[cfg(target_os = "macos")]
mod brew;
#[cfg(target_os = "macos")]
mod mas;
#[cfg(target_os = "macos")]
mod port;
#[cfg(target_os = "macos")]
mod softwareupdate;

#[cfg(target_os = "macos")]
pub use brew::BrewManager;
#[cfg(target_os = "macos")]
pub use mas::MasManager;
#[cfg(target_os = "macos")]
pub use port::PortManager;
#[cfg(target_os = "macos")]
pub use softwareupdate::SoftwareUpdateManager;

// Linux managers
#[cfg(target_os = "linux")]
mod apt;
#[cfg(target_os = "linux")]
mod dnf;
#[cfg(target_os = "linux")]
mod flatpak;
#[cfg(target_os = "linux")]
mod pacman;
#[cfg(target_os = "linux")]
mod snap;

#[cfg(target_os = "linux")]
pub use apt::AptManager;
#[cfg(target_os = "linux")]
pub use dnf::DnfManager;
#[cfg(target_os = "linux")]
pub use flatpak::FlatpakManager;
#[cfg(target_os = "linux")]
pub use pacman::PacmanManager;
#[cfg(target_os = "linux")]
pub use snap::SnapManager;

// Windows managers
#[cfg(target_os = "windows")]
mod choco;
#[cfg(target_os = "windows")]
mod scoop;
#[cfg(target_os = "windows")]
mod winget;

#[cfg(target_os = "windows")]
pub use choco::ChocoManager;
#[cfg(target_os = "windows")]
pub use scoop::ScoopManager;
#[cfg(target_os = "windows")]
pub use winget::WingetManager;

// Version managers (cross-platform)
mod conda;
mod mise;

pub use conda::CondaManager;
pub use mise::MiseManager;

// Language managers (cross-platform)
mod cargo;
mod gem;
mod npm;
mod pipx;
mod pnpm;
mod rustup;

pub use cargo::CargoManager;
pub use gem::GemManager;
pub use npm::NpmManager;
pub use pipx::PipxManager;
pub use pnpm::PnpmManager;
pub use rustup::RustupManager;

/// Create a PackageManager from a Manager enum variant.
/// Returns None if no implementation exists for this Manager.
pub fn create_manager(manager: Manager) -> Option<Box<dyn PackageManager>> {
    match manager {
        // macOS managers
        #[cfg(target_os = "macos")]
        Manager::Brew => Some(Box::new(BrewManager)),
        #[cfg(target_os = "macos")]
        Manager::Port => Some(Box::new(PortManager)),
        #[cfg(target_os = "macos")]
        Manager::Mas => Some(Box::new(MasManager)),
        #[cfg(target_os = "macos")]
        Manager::SoftwareUpdate => Some(Box::new(SoftwareUpdateManager)),

        // Linux managers
        #[cfg(target_os = "linux")]
        Manager::Apt => Some(Box::new(AptManager)),
        #[cfg(target_os = "linux")]
        Manager::Dnf => Some(Box::new(DnfManager)),
        #[cfg(target_os = "linux")]
        Manager::Pacman => Some(Box::new(PacmanManager)),
        #[cfg(target_os = "linux")]
        Manager::Flatpak => Some(Box::new(FlatpakManager)),
        #[cfg(target_os = "linux")]
        Manager::Snap => Some(Box::new(SnapManager)),

        // Windows managers
        #[cfg(target_os = "windows")]
        Manager::Winget => Some(Box::new(WingetManager)),
        #[cfg(target_os = "windows")]
        Manager::Choco => Some(Box::new(ChocoManager)),
        #[cfg(target_os = "windows")]
        Manager::Scoop => Some(Box::new(ScoopManager)),

        // Version managers (cross-platform)
        Manager::Mise => Some(Box::new(MiseManager)),
        Manager::Conda => Some(Box::new(CondaManager)),

        // Language managers (cross-platform)
        Manager::Npm => Some(Box::new(NpmManager)),
        Manager::Pnpm => Some(Box::new(PnpmManager)),
        Manager::Pipx => Some(Box::new(PipxManager)),
        Manager::Gem => Some(Box::new(GemManager)),
        Manager::Rustup => Some(Box::new(RustupManager)),
        Manager::Cargo => Some(Box::new(CargoManager)),

        // No implementation yet or not applicable
        _ => None,
    }
}
