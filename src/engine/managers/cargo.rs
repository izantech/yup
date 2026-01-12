//! cargo package manager

use super::{command_exists, Action, ActionKind, Manager, PackageManager};

/// Cargo - Rust package manager
pub struct CargoManager;

/// Check if cargo-install-update is available
fn cargo_update_available() -> bool {
    command_exists("cargo-install-update")
}

impl PackageManager for CargoManager {
    fn name(&self) -> &'static str {
        "cargo"
    }

    fn update_actions(&self) -> Vec<Action> {
        // Cargo itself is updated via rustup
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Only offer upgrade if cargo-install-update is installed
        if cargo_update_available() {
            vec![Action {
                manager: Manager::Cargo,
                kind: ActionKind::Upgrade,
                command: "cargo install-update -a".to_string(),
                description: "Update all cargo-installed binaries".to_string(),
            }]
        } else {
            vec![]
        }
    }

    fn check_actions(&self) -> Vec<Action> {
        if cargo_update_available() {
            vec![Action {
                manager: Manager::Cargo,
                kind: ActionKind::Check,
                command: "cargo install-update -l".to_string(),
                description: "List outdated cargo-installed binaries".to_string(),
            }]
        } else {
            vec![]
        }
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
