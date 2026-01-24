//! Homebrew package manager

import super.{ Action, Manager, PackageManager }

/// Homebrew package manager
public struct BrewManager

extension BrewManager: PackageManager {
  fn updateActions(): [Action] {
    [Action(manager: Manager.brew, command: "brew update", description: "Update Homebrew formulae", requiresPrivilege: false)]
  }
  fn upgradeActions(): [Action] {
    [
      Action(manager: Manager.brew, command: "brew upgrade", description: "Upgrade outdated formulae", requiresPrivilege: false),
      Action(manager: Manager.brew, command: "brew cleanup", description: "Remove old versions", requiresPrivilege: false),
    ]
  }
  fn checkActions(): [Action] {
    [Action(manager: Manager.brew, command: "brew outdated", description: "Check for outdated formulae", requiresPrivilege: false)]
  }
}
