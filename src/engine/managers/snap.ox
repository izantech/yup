//! Snap package manager

import super.{ Action, Manager, PackageManager }

/// Snap package manager
public struct SnapManager

extension SnapManager: PackageManager {
  fn updateActions(): Vec<Action> {
    // snap refresh does both update and upgrade
    []
  }
  fn upgradeActions(): Vec<Action> {
    [
      Action(
        manager: Manager.snap,
        command: "snap refresh",
        description: "Refresh all Snap packages",
        requiresPrivilege: true,
      ),
    ]
  }
  fn checkActions(): Vec<Action> {
    [
      Action(
        manager: Manager.snap,
        command: "snap refresh --list",
        description: "Check for available updates",
        requiresPrivilege: false,
      ),
    ]
  }
}
