//! cargo package manager

use super::{Action, Manager, PackageManager};

/// Cargo - Rust package manager
pub struct CargoManager;

/// Check if cargo-install-update is available
fn is_cargo_update_available() -> bool {
    which::which("cargo-install-update").is_ok()
}

impl PackageManager for CargoManager {
    fn update_actions(&self) -> Vec<Action> {
        // Cargo itself is updated via rustup
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Only offer upgrade if cargo-install-update is installed
        if is_cargo_update_available() {
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
        if is_cargo_update_available() {
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
