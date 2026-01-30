//! macOS Software Update

import super.{ Action, Manager, PackageManager }

/// macOS Software Update manager
public struct SoftwareUpdateManager

extension SoftwareUpdateManager: PackageManager {
  fn updateActions(): Array<Action> {
    // softwareupdate list is automatic
    []
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.SoftwareUpdate,
        command: "softwareupdate -ia",
        description: "Install all macOS updates (may require restart)",
        requiresPrivilege: true,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.SoftwareUpdate,
        command: "softwareupdate -l",
        description: "Check for available macOS updates",
        requiresPrivilege: false,
      ),
    ]
  }
}
