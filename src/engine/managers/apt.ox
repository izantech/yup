//! APT package manager (Debian/Ubuntu)

import super.{Action, Manager, PackageManager}

/// APT package manager
public struct AptManager

extension AptManager: PackageManager {
    fn updateActions(): [Action] {
        [Action.new(
            Manager.apt,
            "apt update",
            "Update package index",
            true
        )]
    }

    fn upgradeActions(): [Action] {
        [
            Action.new(
                Manager.apt,
                "apt upgrade -y",
                "Upgrade installed packages",
                true
            ),
            Action.new(
                Manager.apt,
                "apt autoremove -y",
                "Remove unused dependencies",
                true
            ),
        ]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.apt,
            "apt list --upgradable",
            "Check for available updates",
            false
        )]
    }
}
