//! Chocolatey package manager

import super.{ Action, Manager, PackageManager }

/// Chocolatey package manager
public struct ChocoManager

extension ChocoManager: PackageManager {
  fn updateActions(): [Action] {
    // choco upgrade does both update and upgrade
    []
  }
  fn upgradeActions(): [Action] {
    [Action.new(Manager.choco, "choco upgrade all -y", "Upgrade all packages", true)]
  }
  fn checkActions(): [Action] {
    [Action.new(Manager.choco, "choco outdated", "Check for outdated packages", false)]
  }
}
