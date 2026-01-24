//! Pacman package manager (Arch Linux)

import super.{ Action, Manager, PackageManager }
import which.which

/// Pacman package manager
public struct PacmanManager

extension PacmanManager: PackageManager {
  fn updateActions(): [Action] {
    // pacman -Syu combines update+upgrade, no separate update needed
    []
  }
  fn upgradeActions(): [Action] {
    [Action(manager: Manager.pacman, command: "pacman -Syu --noconfirm", description: "Sync and upgrade all packages", requiresPrivilege: true)]
  }
  fn checkActions(): [Action] {
    if which("checkupdates").isOk() {
      [Action(manager: Manager.pacman, command: "checkupdates", description: "Check for available updates", requiresPrivilege: false)]
    } else {
      []
    }
  }
}
