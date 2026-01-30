//! pnpm package manager

import super.{ Action, Manager, PackageManager }

/// pnpm package manager
public struct PnpmManager

extension PnpmManager: PackageManager {
  fn updateActions(): Array<Action> {
    [
      Action(
        manager: Manager.Pnpm,
        command: "pnpm self-update",
        description: "Update pnpm itself",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.Pnpm,
        command: "pnpm update -g",
        description: "Update global pnpm packages",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Pnpm,
        command: "pnpm outdated -g",
        description: "Check for outdated global pnpm packages",
        requiresPrivilege: false,
      ),
    ]
  }
}
