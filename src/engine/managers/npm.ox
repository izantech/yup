//! npm package manager

import super.{ Action, Manager, PackageManager }

/// npm package manager
public struct NpmManager

extension NpmManager: PackageManager {
  fn updateActions(): [Action] {
    [Action.new(Manager.npm, "npm install -g npm", "Update npm itself", false)]
  }
  fn upgradeActions(): [Action] {
    [Action.new(Manager.npm, "npm update -g", "Update global npm packages", false)]
  }
  fn checkActions(): [Action] {
    [Action.new(Manager.npm, "npm outdated -g", "Check for outdated global npm packages", false)]
  }
}
