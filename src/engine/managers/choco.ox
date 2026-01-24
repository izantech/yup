//! Chocolatey package manager

import super.{Action, Manager, PackageManager}

/// Chocolatey package manager
public struct ChocoManager

extension ChocoManager: PackageManager {
    fn update_actions(): Vec<Action> {
        // choco upgrade does both update and upgrade
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Choco,
            "choco upgrade all -y",
            "Upgrade all packages",
            true
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Choco,
            "choco outdated",
            "Check for outdated packages",
            false
        )]
    }
}
