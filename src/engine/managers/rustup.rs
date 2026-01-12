//! rustup toolchain manager

use super::{Action, Manager, PackageManager};

/// Rustup - Rust toolchain installer
pub struct RustupManager;

impl PackageManager for RustupManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Rustup,
            "rustup update",
            "Update all installed Rust toolchains",
            false,
        )]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Rustup manages toolchains, not packages
        vec![]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Rustup,
            "rustup check",
            "Check for Rust toolchain updates",
            false,
        )]
    }
}
