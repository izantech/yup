//! rustup toolchain manager

import super.{ Action, Manager, PackageManager }

/// Rustup - Rust toolchain installer
public struct RustupManager

extension RustupManager: PackageManager {
  fn updateActions(): [Action] {
    [Action.new(Manager.rustup, "rustup update", "Update all installed Rust toolchains", false)]
  }
  fn upgradeActions(): [Action] {
    // Rustup manages toolchains, not packages
    []
  }
  fn checkActions(): [Action] {
    [Action.new(Manager.rustup, "rustup check", "Check for Rust toolchain updates", false)]
  }
}
