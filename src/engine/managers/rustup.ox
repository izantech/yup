//! rustup toolchain manager

import super.{ Action, Manager, PackageManager }

/// Rustup - Rust toolchain installer
public struct RustupManager

extension RustupManager: PackageManager {
  fn updateActions(): Array<Action> {
    [
      Action(
        manager: Manager.Rustup,
        command: "rustup update",
        description: "Update all installed Rust toolchains",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Array<Action> {
    // Rustup manages toolchains, not packages
    []
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Rustup,
        command: "rustup check",
        description: "Check for Rust toolchain updates",
        requiresPrivilege: false,
      ),
    ]
  }
}
