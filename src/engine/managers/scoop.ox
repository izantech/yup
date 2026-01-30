//! Scoop package manager

import super.{ Action, Manager, PackageManager }

/// Scoop package manager
public struct ScoopManager

extension ScoopManager: PackageManager {
  fn updateActions(): Array<Action> {
    [
      Action(
        manager: Manager.Scoop,
        command: "scoop update",
        description: "Update Scoop and manifests",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.Scoop,
        command: "scoop update *",
        description: "Update all packages",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Scoop,
        command: "scoop status",
        description: "Check for outdated packages",
        requiresPrivilege: false,
      ),
    ]
  }
}
