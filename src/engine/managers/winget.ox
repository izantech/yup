//! Windows Package Manager (winget)

import super.{ Action, Manager, PackageManager }

/// Windows Package Manager
public struct WingetManager

extension WingetManager: PackageManager {
  fn updateActions(): Array<Action> {
    // winget upgrade does both update and upgrade
    []
  }
  fn upgradeActions(): Array<Action> {
    [
      Action(
        manager: Manager.Winget,
        command: "winget upgrade --all --silent --disable-interactivity --accept-package-agreements --accept-source-agreements",
        description: "Upgrade all packages",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Winget,
        command: "winget list --upgrade-available",
        description: "Check for available upgrades",
        requiresPrivilege: false,
      ),
    ]
  }
}
