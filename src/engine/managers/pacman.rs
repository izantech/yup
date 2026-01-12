//! Pacman package manager (Arch Linux)

use super::{Action, ActionKind, Manager, PackageManager};

/// Pacman package manager
pub struct PacmanManager;

impl PackageManager for PacmanManager {
    fn name(&self) -> &'static str {
        "Pacman"
    }

    fn update_actions(&self) -> Vec<Action> {
        // pacman -Syu combines update+upgrade, no separate update needed
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Pacman,
            kind: ActionKind::Upgrade,
            command: "pacman -Syu --noconfirm".to_string(),
            description: "Sync and upgrade all packages".to_string(),
        }]
    }

    fn requires_privilege(&self) -> bool {
        true
    }
}
