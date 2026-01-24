//! DNF package manager (Fedora/RHEL)

import super.{ Action, Manager, PackageManager }

/// DNF package manager
public struct DnfManager

extension DnfManager: PackageManager {
  fn updateActions(): Array<Action> {
    // DNF auto-syncs during upgrade, no separate update needed
    []
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.dnf,
        command: "dnf upgrade -y",
        description: "Upgrade all packages",
        requiresPrivilege: true,
      ),
      Action(
        manager: Manager.dnf,
        command: "dnf autoremove -y",
        description: "Remove unused dependencies",
        requiresPrivilege: true,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.dnf,
        command: "dnf check-update",
        description: "Check for available updates",
        requiresPrivilege: false,
      ),
    ]
  }
}
