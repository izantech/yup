//! Chocolatey package manager

import super.{ Action, Manager, PackageManager }

/// Chocolatey package manager
public struct ChocoManager

extension ChocoManager: PackageManager {
  fn updateActions(): Array<Action> {
    // choco upgrade does both update and upgrade
    []
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.Choco,
        command: "choco upgrade all -y",
        description: "Upgrade all packages",
        requiresPrivilege: true,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Choco,
        command: "choco outdated",
        description: "Check for outdated packages",
        requiresPrivilege: false,
      ),
    ]
  }
}
