//! DNF package manager (Fedora/RHEL)

import super.{Action, Manager, PackageManager}

/// DNF package manager
public struct DnfManager

extension DnfManager: PackageManager {
    fn updateActions(): [Action] {
        // DNF auto-syncs during upgrade, no separate update needed
        []
    }

    fn upgradeActions(): [Action] {
        [
            Action.new(Manager.dnf, "dnf upgrade -y", "Upgrade all packages", true),
            Action.new(
                Manager.dnf,
                "dnf autoremove -y",
                "Remove unused dependencies",
                true
            ),
        ]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.dnf,
            "dnf check-update",
            "Check for available updates",
            false
        )]
    }
}
