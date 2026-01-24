//! Pacman package manager (Arch Linux)

import super.{Action, Manager, PackageManager}
import which.which

/// Pacman package manager
public struct PacmanManager

extension PacmanManager: PackageManager {
    fn updateActions(): [Action] {
        // pacman -Syu combines update+upgrade, no separate update needed
        []
    }

    fn upgradeActions(): [Action] {
        [Action.new(
            Manager.pacman,
            "pacman -Syu --noconfirm",
            "Sync and upgrade all packages",
            true
        )]
    }

    fn checkActions(): [Action] {
        if which("checkupdates").is_ok() {
            [Action.new(
                Manager.pacman,
                "checkupdates",
                "Check for available updates",
                false
            )]
        } else {
            []
        }
    }
}
