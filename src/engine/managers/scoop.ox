//! Scoop package manager

import super.{ Action, Manager, PackageManager }

/// Scoop package manager
public struct ScoopManager

extension ScoopManager: PackageManager {
  fn updateActions(): [Action] {
    [Action.new(Manager.scoop, "scoop update", "Update Scoop and manifests", false)]
  }
  fn upgradeActions(): [Action] {
    [Action.new(Manager.scoop, "scoop update *", "Update all packages", false)]
  }
  fn checkActions(): [Action] {
    [Action.new(Manager.scoop, "scoop status", "Check for outdated packages", false)]
  }
}
