//! Cargo package manager

import std.process.Command
import super.{ Action, Manager, PackageManager }

/// cargo package manager
public struct CargoManager

/// Check if cargo-update is installed
fn isCargoUpdateAvailable(): bool {
  Command.new("cargo").args(["install", "--list"]).output().map { output ->
    let stdout = String.fromUtf8Lossy(output.stdout)
    stdout.contains("cargo-update")
  }.unwrapOr(false)
}

extension CargoManager: PackageManager {
  fn updateActions(): [Action] { [] }
  fn upgradeActions(): [Action] {
    if isCargoUpdateAvailable() {
      [Action.new(Manager.cargo, "cargo install-update -a", "Update all cargo-installed binaries", false)]
    } else {
      []
    }
  }
  fn checkActions(): [Action] {
    if isCargoUpdateAvailable() {
      [Action.new(Manager.cargo, "cargo install-update -l", "Check for outdated cargo binaries", false)]
    } else {
      []
    }
  }
}
