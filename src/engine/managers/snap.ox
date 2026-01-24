//! Snap package manager

import super.{Action, Manager, PackageManager}

/// Snap package manager
public struct SnapManager

extension SnapManager: PackageManager {
    fn update_actions(): Vec<Action> {
        // snap refresh does both update and upgrade
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Snap,
            "snap refresh",
            "Refresh all Snap packages",
            true
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Snap,
            "snap refresh --list",
            "Check for available updates",
            false
        )]
    }
}
