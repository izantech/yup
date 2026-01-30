//! Pacman package manager (Arch Linux)

import super.{ Action, Manager, PackageManager }
import which.which

/// Pacman package manager
public struct PacmanManager

extension PacmanManager: PackageManager {
  fn updateActions(): Array<Action> {
    // pacman -Syu combines update+upgrade, no separate update needed
    []
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.Pacman,
        command: "pacman -Syu --noconfirm",
        description: "Sync and upgrade all packages",
        requiresPrivilege: true,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    if which("checkupdates").isOk() {
      [
        Action(
          manager: Manager.Pacman,
          command: "checkupdates",
          description: "Check for available updates",
          requiresPrivilege: false,
        ),
      ]
    } else {
      []
    }
  }
}
