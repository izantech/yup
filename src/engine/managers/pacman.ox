//! Pacman package manager (Arch Linux)

import super.{Action, Manager, PackageManager}

/// Pacman package manager
public struct PacmanManager

extension PacmanManager: PackageManager {
    fn update_actions(): Vec<Action> {
        // pacman -Syu combines update+upgrade, no separate update needed
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Pacman,
            "pacman -Syu --noconfirm",
            "Sync and upgrade all packages",
            true
        )]
    }

    fn check_actions(): Vec<Action> {
        if which.which("checkupdates").is_ok() {
            vec![Action.new(
                Manager.Pacman,
                "checkupdates",
                "Check for available updates",
                false
            )]
        } else {
            vec![]
        }
    }
}
