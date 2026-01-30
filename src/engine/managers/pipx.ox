//! pipx package manager

import super.{ Action, Manager, PackageManager }

/// pipx - install and run Python applications in isolated environments
public struct PipxManager

extension PipxManager: PackageManager {
  fn updateActions(): Array<Action> {
    // pipx is typically managed by pip or brew, no self-update command
    []
  }
  fn upgradeActions(): Array<Action> {
    // pipx upgrade-all is safe because each app is in its own isolated venv
    [
      Action(
        manager: Manager.Pipx,
        command: "pipx upgrade-all",
        description: "Upgrade all pipx-installed packages",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    // pipx doesn't provide a read-only "outdated" or dry-run upgrade command.
    []
  }
}
