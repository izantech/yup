//! conda package/environment manager

import super.{ Action, Manager, PackageManager }

/// conda package manager
public struct CondaManager

extension CondaManager: PackageManager {
  fn updateActions(): Array<Action> {
    [
      Action(
        manager: Manager.Conda,
        command: "conda update -n base conda -y",
        description: "Update conda itself",
        requiresPrivilege: false,
      ),
    ]
  }
  fn upgradeActions(): Array<Action> {
    // Upgrading all base packages can break environments
    [
      Action(
        manager: Manager.Conda,
        command: "conda update -n base --all -y",
        description: "Update all packages in base environment",
        requiresPrivilege: false,
      ),
    ]
  }
  fn checkActions(): Array<Action> {
    [
      Action(
        manager: Manager.Conda,
        command: "conda list --outdated",
        description: "Check for outdated packages",
        requiresPrivilege: false,
      ),
    ]
  }
}
