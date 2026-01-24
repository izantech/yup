//! macOS Software Update

import super.{Action, Manager, PackageManager}

/// macOS Software Update manager
public struct SoftwareUpdateManager

extension SoftwareUpdateManager: PackageManager {
    fn update_actions(): Vec<Action> {
        // softwareupdate list is automatic
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.SoftwareUpdate,
            "softwareupdate -ia",
            "Install all macOS updates (may require restart)",
            true
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.SoftwareUpdate,
            "softwareupdate -l",
            "Check for available macOS updates",
            false
        )]
    }
}
