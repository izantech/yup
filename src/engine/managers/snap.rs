//! Snap package manager

use super::{Action, Manager, PackageManager};

/// Snap package manager
pub struct SnapManager;

impl PackageManager for SnapManager {
    fn update_actions(&self) -> Vec<Action> {
        // snap refresh does both update and upgrade
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Snap,
            "snap refresh",
            "Refresh all Snap packages",
            true,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Snap,
            "snap refresh --list",
            "Check for available updates",
            false,
        )]
    }
}
