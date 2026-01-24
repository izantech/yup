//! MacPorts package manager

import super.{ Action, Manager, PackageManager }

/// MacPorts package manager
public struct PortManager

extension PortManager: PackageManager {
  fn updateActions(): [Action] {
    [Action.new(Manager.port, "port selfupdate", "Update MacPorts and port definitions", true)]
  }
  fn upgradeActions(): [Action] {
    [Action.new(Manager.port, "port upgrade outdated", "Upgrade outdated ports", true)]
  }
  fn checkActions(): [Action] {
    [Action.new(Manager.port, "port outdated", "Check for outdated ports", false)]
  }
}
