//! MacPorts package manager

import super.{ Action, Manager, PackageManager }

/// MacPorts package manager
public struct PortManager

extension PortManager: PackageManager {
  fn updateActions(): Vec<Action> {
    [
      Action(
        manager: Manager.port,
        command: "port selfupdate",
        description: "Update MacPorts and port definitions",
        requiresPrivilege: true,
      ),
    ]
  }
  fn upgradeActions(): Vec<Action> {
    [
      Action(
        manager: Manager.port,
        command: "port upgrade outdated",
        description: "Upgrade outdated ports",
        requiresPrivilege: true,
      ),
    ]
  }
  fn checkActions(): Vec<Action> {
    [
      Action(
        manager: Manager.port,
        command: "port outdated",
        description: "Check for outdated ports",
        requiresPrivilege: false,
      ),
    ]
  }
}
