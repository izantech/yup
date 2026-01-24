//! conda package/environment manager

import super.{Action, Manager, PackageManager}

/// conda package manager
public struct CondaManager

extension CondaManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Conda,
            "conda update -n base conda -y",
            "Update conda itself",
            false
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        // Upgrading all base packages can break environments
        vec![Action.new(
            Manager.Conda,
            "conda update -n base --all -y",
            "Update all packages in base environment",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Conda,
            "conda list --outdated",
            "Check for outdated packages",
            false
        )]
    }
}
