//! Mac App Store CLI (mas)

import super.{ Action, Manager, PackageManager }

/// Mac App Store CLI manager
public struct MasManager

extension MasManager: PackageManager {
  fn updateActions(): [Action] {
    // mas has no separate update step
    []
  }
  fn upgradeActions(): [Action] {
    [Action(manager: Manager.mas, command: "mas upgrade", description: "Upgrade Mac App Store apps", requiresPrivilege: true)]
  }
  fn checkActions(): [Action] {
    [Action(manager: Manager.mas, command: "mas outdated", description: "Check for outdated Mac App Store apps", requiresPrivilege: false)]
  }
}
