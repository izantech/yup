//! DNF package manager (Fedora/RHEL)

import super.{Action, Manager, PackageManager}

/// DNF package manager
public struct DnfManager

extension DnfManager: PackageManager {
    fn update_actions(): Vec<Action> {
        // DNF auto-syncs during upgrade, no separate update needed
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![
            Action.new(Manager.Dnf, "dnf upgrade -y", "Upgrade all packages", true),
            Action.new(
                Manager.Dnf,
                "dnf autoremove -y",
                "Remove unused dependencies",
                true
            ),
        ]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Dnf,
            "dnf check-update",
            "Check for available updates",
            false
        )]
    }
}
