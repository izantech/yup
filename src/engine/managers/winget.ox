//! Windows Package Manager (winget)

import super.{Action, Manager, PackageManager}

/// Windows Package Manager
public struct WingetManager

extension WingetManager: PackageManager {
    fn update_actions(): Vec<Action> {
        // winget upgrade does both update and upgrade
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Winget,
            "winget upgrade --all --silent --disable-interactivity --accept-package-agreements --accept-source-agreements",
            "Upgrade all packages",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Winget,
            "winget list --upgrade-available",
            "Check for available upgrades",
            false
        )]
    }
}
