//! Windows Package Manager (winget)

import super.{Action, Manager, PackageManager}

/// Windows Package Manager
public struct WingetManager

extension WingetManager: PackageManager {
    fn updateActions(): [Action] {
        // winget upgrade does both update and upgrade
        []
    }

    fn upgradeActions(): [Action] {
        [Action.new(
            Manager.winget,
            "winget upgrade --all --silent --disable-interactivity --accept-package-agreements --accept-source-agreements",
            "Upgrade all packages",
            false
        )]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.winget,
            "winget list --upgrade-available",
            "Check for available upgrades",
            false
        )]
    }
}
