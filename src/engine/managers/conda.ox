//! conda package/environment manager

import super.{Action, Manager, PackageManager}

/// conda package manager
public struct CondaManager

extension CondaManager: PackageManager {
    fn updateActions(): [Action] {
        [Action.new(
            Manager.conda,
            "conda update -n base conda -y",
            "Update conda itself",
            false
        )]
    }

    fn upgradeActions(): [Action] {
        // Upgrading all base packages can break environments
        [Action.new(
            Manager.conda,
            "conda update -n base --all -y",
            "Update all packages in base environment",
            false
        )]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.conda,
            "conda list --outdated",
            "Check for outdated packages",
            false
        )]
    }
}
