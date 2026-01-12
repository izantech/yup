//! Pacman package manager (Arch Linux)

use super::{Action, Manager, PackageManager};

/// Pacman package manager
pub struct PacmanManager;

impl PackageManager for PacmanManager {
    fn update_actions(&self) -> Vec<Action> {
        // pacman -Syu combines update+upgrade, no separate update needed
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Pacman,
            "pacman -Syu --noconfirm",
            "Sync and upgrade all packages",
            true,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Pacman,
            "checkupdates",
            "Check for available updates",
            false,
        )]
    }
}
