//! APT package manager (Debian/Ubuntu)

import super.{Action, Manager, PackageManager}

/// APT package manager
public struct AptManager

extension AptManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Apt,
            "apt update",
            "Update package index",
            true
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![
            Action.new(
                Manager.Apt,
                "apt upgrade -y",
                "Upgrade installed packages",
                true
            ),
            Action.new(
                Manager.Apt,
                "apt autoremove -y",
                "Remove unused dependencies",
                true
            ),
        ]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Apt,
            "apt list --upgradable",
            "Check for available updates",
            false
        )]
    }
}
