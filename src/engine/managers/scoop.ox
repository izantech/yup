//! Scoop package manager

import super.{ Action, Manager, PackageManager }

/// Scoop package manager
public struct ScoopManager

extension ScoopManager: PackageManager {
  fn updateActions(): Vec<Action> {
    [
      Action(
        manager: Manager.scoop,
        command: "scoop update",
        description: "Update Scoop and manifests",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Vec<Action> {
    [
      Action(
        manager: Manager.scoop,
        command: "scoop update *",
        description: "Update all packages",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Vec<Action> {
    [
      Action(
        manager: Manager.scoop,
        command: "scoop status",
        description: "Check for outdated packages",
        requiresPrivilege: false,
      ),
    ]
  }
}
