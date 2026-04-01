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

/// Declarative macro for defining package manager modules with less boilerplate.
///
/// This macro generates:
/// 1. Module declarations with appropriate cfg attributes
/// 2. Public re-exports of manager structs
/// 3. The entire `create_manager()` factory function
///
/// # Usage
/// ```ignore
/// declare_managers! {
///     // Cross-platform managers (no attributes)
///     Cargo => cargo::CargoManager,
///     Npm   => npm::NpmManager,
///
///     // Platform-specific managers (with cfg attributes)
///     #[cfg(target_os = "macos")]
///     Brew => brew::BrewManager,
///
///     #[cfg(target_os = "linux")]
///     Apt => apt::AptManager,
/// }
/// ```
macro_rules! declare_managers {
    (
        $(
            $(#[$meta:meta])*
            $variant:ident => $mod_name:ident :: $struct_name:ident
        ),+ $(,)?
    ) => {
        $(
            $(#[$meta])*
            mod $mod_name;

            $(#[$meta])*
            pub use $mod_name::$struct_name;
        )+

        /// Create a PackageManager from a Manager enum variant.
        /// Returns None if no implementation exists for this Manager.
        pub fn create_manager(manager: Manager) -> Option<Box<dyn PackageManager>> {
            match manager {
                $(
                    $(#[$meta])*
                    Manager::$variant => Some(Box::new($struct_name)),
                )+
                _ => None,
            }
        }
    };
}

// Package managers - all platforms
declare_managers! {
    // Cross-platform managers (8 total)
    Conda  => conda::CondaManager,
    Mise   => mise::MiseManager,
    Cargo  => cargo::CargoManager,
    Gem    => gem::GemManager,
    Npm    => npm::NpmManager,
    Pipx   => pipx::PipxManager,
    Pnpm   => pnpm::PnpmManager,
    Rustup => rustup::RustupManager,

    // macOS managers (4 total)
    #[cfg(target_os = "macos")]
    Brew => brew::BrewManager,
    #[cfg(target_os = "macos")]
    Mas => mas::MasManager,
    #[cfg(target_os = "macos")]
    Port => port::PortManager,
    #[cfg(target_os = "macos")]
    SoftwareUpdate => softwareupdate::SoftwareUpdateManager,

    // Linux managers (5 total)
    #[cfg(target_os = "linux")]
    Apt => apt::AptManager,
    #[cfg(target_os = "linux")]
    Dnf => dnf::DnfManager,
    #[cfg(target_os = "linux")]
    Flatpak => flatpak::FlatpakManager,
    #[cfg(target_os = "linux")]
    Pacman => pacman::PacmanManager,
    #[cfg(target_os = "linux")]
    Snap => snap::SnapManager,

    // Windows managers (3 total)
    #[cfg(target_os = "windows")]
    Choco => choco::ChocoManager,
    #[cfg(target_os = "windows")]
    Scoop => scoop::ScoopManager,
    #[cfg(target_os = "windows")]
    Winget => winget::WingetManager,
}

/// Append `--greedy` to any `brew upgrade` action that doesn't already have it.
pub fn apply_brew_greedy(actions: Vec<Action>) -> Vec<Action> {
    actions
        .into_iter()
        .map(|mut a| {
            if a.manager == Manager::Brew
                && a.command.starts_with("brew upgrade")
                && !a.command.contains("--greedy")
            {
                a.command.push_str(" --greedy");
            }
            a
        })
        .collect()
}

/// Append `--yes` to any `mise self-update` action that doesn't already have it.
pub fn apply_mise_yes(actions: Vec<Action>) -> Vec<Action> {
    actions
        .into_iter()
        .map(|mut a| {
            if a.manager == Manager::Mise
                && a.command.starts_with("mise self-update")
                && !a.command.contains("--yes")
            {
                a.command.push_str(" --yes");
            }
            a
        })
        .collect()
}
