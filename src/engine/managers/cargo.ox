//! Cargo package manager

import std.process.Command
import super.{ Action, Manager, PackageManager }

/// cargo package manager
public struct CargoManager

/// Check if cargo-update is installed
fn isCargoUpdateAvailable(): bool {
  Command("cargo")
    .args(["install", "--list"])
    .output()
    .map { output ->
      let stdout = String.fromUtf8Lossy(output.stdout)
      stdout.contains("cargo-update")
    }
    .unwrapOr(false)
}

extension CargoManager: PackageManager {
  fn updateActions(): Vec<Action> { [] }
  fn upgradeActions(): Vec<Action> {
    if isCargoUpdateAvailable() {
      [
        Action(
          manager: Manager.cargo,
          command: "cargo install-update -a",
          description: "Update all cargo-installed binaries",
          requiresPrivilege: false,
        ),
      ]
    } else {
      []
    }
  }
  fn checkActions(): Vec<Action> {
    if isCargoUpdateAvailable() {
      [
        Action(
          manager: Manager.cargo,
          command: "cargo install-update -l",
          description: "Check for outdated cargo binaries",
          requiresPrivilege: false,
        ),
      ]
    } else {
      []
    }
  }
}
