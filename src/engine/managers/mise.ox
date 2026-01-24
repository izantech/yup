//! mise version manager (formerly rtx)

import super.{ Action, Manager, PackageManager }

/// mise version manager - a modern alternative to asdf
public struct MiseManager

extension MiseManager: PackageManager {
  fn updateActions(): [Action] {
    [
      Action(manager: Manager.mise, command: "mise self-update", description: "Update mise itself", requiresPrivilege: false),
      Action(manager: Manager.mise, command: "mise plugins update", description: "Update all mise plugins", requiresPrivilege: false),
    ]
  }
  fn upgradeActions(): [Action] {
    [Action(manager: Manager.mise, command: "mise upgrade", description: "Upgrade all mise-managed tools", requiresPrivilege: false)]
  }
  fn checkActions(): [Action] {
    [Action(manager: Manager.mise, command: "mise outdated", description: "Check for outdated mise-managed tools", requiresPrivilege: false)]
  }
}
