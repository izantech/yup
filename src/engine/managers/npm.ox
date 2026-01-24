//! npm package manager

import super.{ Action, Manager, PackageManager }

/// npm package manager
public struct NpmManager

extension NpmManager: PackageManager {
  fn updateActions(): Array<Action> {
    [
      Action(
        manager: Manager.npm,
        command: "npm install -g npm",
        description: "Update npm itself",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.npm,
        command: "npm update -g",
        description: "Update global npm packages",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.npm,
        command: "npm outdated -g",
        description: "Check for outdated global npm packages",
        requiresPrivilege: false,
      ),
    ]
  }
}
