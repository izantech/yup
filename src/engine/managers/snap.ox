//! Snap package manager

import super.{ Action, Manager, PackageManager }

/// Snap package manager
public struct SnapManager

extension SnapManager: PackageManager {
  fn updateActions(): Array<Action> {
    // snap refresh does both update and upgrade
    []
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.Snap,
        command: "snap refresh",
        description: "Refresh all Snap packages",
        requiresPrivilege: true,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Snap,
        command: "snap refresh --list",
        description: "Check for available updates",
        requiresPrivilege: false,
      ),
    ]
  }
}
