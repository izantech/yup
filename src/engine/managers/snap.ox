//! Snap package manager

import super.{Action, Manager, PackageManager}

/// Snap package manager
public struct SnapManager

extension SnapManager: PackageManager {
    fn updateActions(): [Action] {
        // snap refresh does both update and upgrade
        []
    }

    fn upgradeActions(): [Action] {
        [Action.new(
            Manager.snap,
            "snap refresh",
            "Refresh all Snap packages",
            true
        )]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.snap,
            "snap refresh --list",
            "Check for available updates",
            false
        )]
    }
}
