//! Cargo package manager

import std.process.Command

import super.{Action, Manager, PackageManager}

/// cargo package manager
public struct CargoManager

/// Check if cargo-update is installed
fn is_cargo_update_available(): bool {
    Command.new("cargo")
        .args(["install", "--list"])
        .output()
        .map({ output ->
            let stdout = String.from_utf8_lossy(&output.stdout)
            stdout.contains("cargo-update")
        })
        .unwrap_or(false)
}

extension CargoManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        if is_cargo_update_available() {
            vec![Action.new(
                Manager.Cargo,
                "cargo install-update -a",
                "Update all cargo-installed binaries",
                false
            )]
        } else {
            vec![]
        }
    }

    fn check_actions(): Vec<Action> {
        if is_cargo_update_available() {
            vec![Action.new(
                Manager.Cargo,
                "cargo install-update -l",
                "Check for outdated cargo binaries",
                false
            )]
        } else {
            vec![]
        }
    }
}
