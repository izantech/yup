//! mise version manager (formerly rtx)

import super.{ Action, Manager, PackageManager }

/// mise version manager - a modern alternative to asdf
public struct MiseManager

extension MiseManager: PackageManager {
  fn updateActions(): Array<Action> {
    [
      Action(
        manager: Manager.Mise,
        command: "mise self-update",
        description: "Update mise itself",
        requiresPrivilege: false,
      ),
      Action(
        manager: Manager.Mise,
        command: "mise plugins update",
        description: "Update all mise plugins",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.Mise,
        command: "mise upgrade",
        description: "Upgrade all mise-managed tools",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Mise,
        command: "mise outdated",
        description: "Check for outdated mise-managed tools",
        requiresPrivilege: false,
      ),
    ]
  }
}
