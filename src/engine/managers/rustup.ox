//! rustup toolchain manager

import super.{Action, Manager, PackageManager}

/// Rustup - Rust toolchain installer
public struct RustupManager

extension RustupManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Rustup,
            "rustup update",
            "Update all installed Rust toolchains",
            false
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        // Rustup manages toolchains, not packages
        vec![]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Rustup,
            "rustup check",
            "Check for Rust toolchain updates",
            false
        )]
    }
}
