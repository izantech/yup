//! pnpm package manager

import super.{ Action, Manager, PackageManager }

/// pnpm package manager
public struct PnpmManager

extension PnpmManager: PackageManager {
  fn updateActions(): Vec<Action> {
    [
      Action(
        manager: Manager.pnpm,
        command: "pnpm self-update",
        description: "Update pnpm itself",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Vec<Action> {
    [
      Action(
        manager: Manager.pnpm,
        command: "pnpm update -g",
        description: "Update global pnpm packages",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Vec<Action> {
    [
      Action(
        manager: Manager.pnpm,
        command: "pnpm outdated -g",
        description: "Check for outdated global pnpm packages",
        requiresPrivilege: false,
      ),
    ]
  }
}
