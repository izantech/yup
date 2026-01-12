//! rustup toolchain manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Rustup - Rust toolchain installer
pub struct RustupManager;

impl PackageManager for RustupManager {
    fn name(&self) -> &'static str {
        "rustup"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Rustup,
            kind: ActionKind::Update,
            command: "rustup update".to_string(),
            description: "Update all installed Rust toolchains".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Rustup manages toolchains, not packages
        vec![]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Rustup,
            kind: ActionKind::Check,
            command: "rustup check".to_string(),
            description: "Check for Rust toolchain updates".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
