//! rustup toolchain manager

import super.{ Action, Manager, PackageManager }

/// Rustup - Rust toolchain installer
public struct RustupManager

extension RustupManager: PackageManager {
  fn updateActions(): [Action] {
    [
      Action(
        manager: Manager.rustup,
        command: "rustup update",
        description: "Update all installed Rust toolchains",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): [Action] {
    // Rustup manages toolchains, not packages
    []
  }
  fn checkActions(): [Action] {
    [
      Action(
        manager: Manager.rustup,
        command: "rustup check",
        description: "Check for Rust toolchain updates",
        requiresPrivilege: false,
      ),
    ]
  }
}
