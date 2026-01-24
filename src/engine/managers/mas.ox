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
    [Action.new(Manager.mas, "mas upgrade", "Upgrade Mac App Store apps", true)]
  }
  fn checkActions(): [Action] {
    [Action.new(Manager.mas, "mas outdated", "Check for outdated Mac App Store apps", false)]
  }
}
