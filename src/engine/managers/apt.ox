//! APT package manager (Debian/Ubuntu)

import super.{ Action, Manager, PackageManager }

/// APT package manager
public struct AptManager

extension AptManager: PackageManager {
  fn updateActions(): Vec<Action> {
    [
      Action(
        manager: Manager.apt,
        command: "apt update",
        description: "Update package index",
        requiresPrivilege: true,
      ),
    ]
  }
  fn upgradeActions(): Vec<Action> {
    [
      Action(
        manager: Manager.apt,
        command: "apt upgrade -y",
        description: "Upgrade installed packages",
        requiresPrivilege: true,
      ),
      Action(
        manager: Manager.apt,
        command: "apt autoremove -y",
        description: "Remove unused dependencies",
        requiresPrivilege: true,
      ),
    ]
  }
  fn checkActions(): Vec<Action> {
    [
      Action(
        manager: Manager.apt,
        command: "apt list --upgradable",
        description: "Check for available updates",
        requiresPrivilege: false,
      ),
    ]
  }
}
