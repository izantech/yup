//! Chocolatey package manager

import super.{ Action, Manager, PackageManager }

/// Chocolatey package manager
public struct ChocoManager

extension ChocoManager: PackageManager {
  fn updateActions(): Vec<Action> {
    // choco upgrade does both update and upgrade
    []
  }
  fn upgradeActions(): Vec<Action> {
    [
      Action(
        manager: Manager.choco,
        command: "choco upgrade all -y",
        description: "Upgrade all packages",
        requiresPrivilege: true,
      ),
    ]
  }
  fn checkActions(): Vec<Action> {
    [
      Action(
        manager: Manager.choco,
        command: "choco outdated",
        description: "Check for outdated packages",
        requiresPrivilege: false,
      ),
    ]
  }
}
