//! gem package manager (RubyGems)

import super.{ Action, Manager, PackageManager }

/// RubyGems package manager
public struct GemManager

extension GemManager: PackageManager {
  fn updateActions(): Array<Action> {
    [
      Action(
        manager: Manager.Gem,
        command: "gem update --system",
        description: "Update RubyGems itself",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.Gem,
        command: "gem update",
        description: "Update all installed gems",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Gem,
        command: "gem outdated",
        description: "Check for outdated Ruby gems",
        requiresPrivilege: false,
      ),
    ]
  }
}
