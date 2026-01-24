//! macOS Software Update

import super.{Action, Manager, PackageManager}

/// macOS Software Update manager
public struct SoftwareUpdateManager

extension SoftwareUpdateManager: PackageManager {
    fn updateActions(): [Action] {
        // softwareupdate list is automatic
        []
    }

    fn upgradeActions(): [Action] {
        [Action.new(
            Manager.softwareupdate,
            "softwareupdate -ia",
            "Install all macOS updates (may require restart)",
            true
        )]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.softwareupdate,
            "softwareupdate -l",
            "Check for available macOS updates",
            false
        )]
    }
}
