//! cargo package manager

use super::{command_exists, Action, Manager, PackageManager};

/// Cargo - Rust package manager
pub struct CargoManager;

/// Check if cargo-install-update is available
fn cargo_update_available() -> bool {
    command_exists("cargo-install-update")
}

impl PackageManager for CargoManager {
    fn update_actions(&self) -> Vec<Action> {
        // Cargo itself is updated via rustup
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Only offer upgrade if cargo-install-update is installed
        if cargo_update_available() {
            vec![Action::new(
                Manager::Cargo,
                "cargo install-update -a",
                "Update all cargo-installed binaries",
                false,
            )]
        } else {
            vec![]
        }
    }

    fn check_actions(&self) -> Vec<Action> {
        if cargo_update_available() {
            vec![Action::new(
                Manager::Cargo,
                "cargo install-update -l",
                "List outdated cargo-installed binaries",
                false,
            )]
        } else {
            vec![]
        }
    }
}
