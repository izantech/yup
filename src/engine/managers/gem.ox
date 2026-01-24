//! gem package manager (RubyGems)

import super.{ Action, Manager, PackageManager }

/// RubyGems package manager
public struct GemManager

extension GemManager: PackageManager {
  fn updateActions(): [Action] {
    [Action.new(Manager.gem, "gem update --system", "Update RubyGems itself", false)]
  }
  fn upgradeActions(): [Action] {
    [Action.new(Manager.gem, "gem update", "Update all installed gems", false)]
  }
  fn checkActions(): [Action] {
    [Action.new(Manager.gem, "gem outdated", "Check for outdated Ruby gems", false)]
  }
}
