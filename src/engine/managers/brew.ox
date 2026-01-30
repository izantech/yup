//! Homebrew package manager

import super.{ Action, Manager, PackageManager }

/// Homebrew package manager
public struct BrewManager

extension BrewManager: PackageManager {
  fn updateActions(): Array<Action> {
    [
      Action(
        manager: Manager.Brew,
        command: "brew update",
        description: "Update Homebrew formulae",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.Brew,
        command: "brew upgrade",
        description: "Upgrade outdated formulae",
        requiresPrivilege: false,
      ),
      Action(
        manager: Manager.Brew,
        command: "brew cleanup",
        description: "Remove old versions",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Brew,
        command: "brew outdated",
        description: "Check for outdated formulae",
        requiresPrivilege: false,
      ),
    ]
  }
}
