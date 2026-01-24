//! Flatpak package manager

import super.{ Action, Manager, PackageManager }

/// Flatpak package manager
public struct FlatpakManager

extension FlatpakManager: PackageManager {
  fn updateActions(): [Action] {
    // flatpak update does both update and upgrade
    []
  }
  fn upgradeActions(): [Action] {
    [Action(manager: Manager.flatpak, command: "flatpak update -y", description: "Update all Flatpak applications", requiresPrivilege: false)]
  }
  fn checkActions(): [Action] {
    [Action(manager: Manager.flatpak, command: "flatpak remote-ls --updates", description: "Check for available updates", requiresPrivilege: false)]
  }
}
