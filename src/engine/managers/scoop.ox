//! Scoop package manager

import super.{ Action, Manager, PackageManager }

/// Scoop package manager
public struct ScoopManager

extension ScoopManager: PackageManager {
  fn updateActions(): [Action] {
    [Action(manager: Manager.scoop, command: "scoop update", description: "Update Scoop and manifests", requiresPrivilege: false)]
  }
  fn upgradeActions(): [Action] {
    [Action(manager: Manager.scoop, command: "scoop update *", description: "Update all packages", requiresPrivilege: false)]
  }
  fn checkActions(): [Action] {
    [Action(manager: Manager.scoop, command: "scoop status", description: "Check for outdated packages", requiresPrivilege: false)]
  }
}
