//! Package manager implementations
//!
//! This module defines the `PackageManager` trait and implementations for
//! various system package managers.

use super::types::{Action, ActionKind, Manager};

/// Trait for package manager implementations
#[allow(dead_code)] // Methods kept for future display/privilege features
pub trait PackageManager {
    /// Human-readable name for display
    fn name(&self) -> &'static str;

    /// Get actions to update package metadata/index (e.g., brew update)
    fn update_actions(&self) -> Vec<Action>;

    /// Get actions to upgrade installed packages (e.g., brew upgrade)
    fn upgrade_actions(&self) -> Vec<Action>;

    /// Get actions to check for outdated packages (e.g., brew outdated)
    fn check_actions(&self) -> Vec<Action> {
        vec![] // Default: no check action
    }

    /// Whether this manager requires root/admin privileges
    fn requires_privilege(&self) -> bool;
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
mod asdf;
mod conda;
mod fnm;
mod mise;
mod nvm;
mod pyenv;
mod rbenv;
mod rvm;
mod volta;

pub use asdf::AsdfManager;
pub use conda::CondaManager;
pub use fnm::FnmManager;
pub use mise::MiseManager;
pub use nvm::NvmManager;
pub use pyenv::PyenvManager;
pub use rbenv::RbenvManager;
pub use rvm::RvmManager;
pub use volta::VoltaManager;

// Language managers (cross-platform)
mod bun;
mod cargo;
mod composer;
mod gem;
mod go;
mod helm;
mod krew;
mod npm;
mod pip;
mod pipx;
mod pnpm;
mod poetry;
mod rustup;
mod sdkman;
mod uv;
mod yarn;

pub use bun::BunManager;
pub use cargo::CargoManager;
pub use composer::ComposerManager;
pub use gem::GemManager;
pub use go::GoManager;
pub use helm::HelmManager;
pub use krew::KrewManager;
pub use npm::NpmManager;
pub use pip::PipManager;
pub use pipx::PipxManager;
pub use pnpm::PnpmManager;
pub use poetry::PoetryManager;
pub use rustup::RustupManager;
pub use sdkman::SdkmanManager;
pub use uv::UvManager;
pub use yarn::YarnManager;

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
        Manager::Asdf => Some(Box::new(AsdfManager)),
        Manager::Mise => Some(Box::new(MiseManager)),
        Manager::Pyenv => Some(Box::new(PyenvManager)),
        Manager::Rbenv => Some(Box::new(RbenvManager)),
        Manager::Rvm => Some(Box::new(RvmManager)),
        Manager::Nvm => Some(Box::new(NvmManager)),
        Manager::Fnm => Some(Box::new(FnmManager)),
        Manager::Volta => Some(Box::new(VoltaManager)),
        Manager::Conda => Some(Box::new(CondaManager)),

        // Language managers (cross-platform)
        Manager::Npm => Some(Box::new(NpmManager)),
        Manager::Pnpm => Some(Box::new(PnpmManager)),
        Manager::Yarn => Some(Box::new(YarnManager)),
        Manager::Bun => Some(Box::new(BunManager)),
        Manager::Pip => Some(Box::new(PipManager)),
        Manager::Pipx => Some(Box::new(PipxManager)),
        Manager::Poetry => Some(Box::new(PoetryManager)),
        Manager::Uv => Some(Box::new(UvManager)),
        Manager::Gem => Some(Box::new(GemManager)),
        Manager::Rustup => Some(Box::new(RustupManager)),
        Manager::Cargo => Some(Box::new(CargoManager)),
        Manager::Go => Some(Box::new(GoManager)),
        Manager::Composer => Some(Box::new(ComposerManager)),
        Manager::Helm => Some(Box::new(HelmManager)),
        Manager::Krew => Some(Box::new(KrewManager)),
        Manager::Sdkman => Some(Box::new(SdkmanManager)),

        // No implementation yet or not applicable
        _ => None,
    }
}
