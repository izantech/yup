//! Scoop package manager

use super::{Action, Manager, PackageManager};

/// Scoop package manager
pub struct ScoopManager;

impl PackageManager for ScoopManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Scoop,
            "scoop update",
            "Update Scoop and manifests",
            false,
        )]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Scoop,
            "scoop update *",
            "Update all packages",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Scoop,
            "scoop status",
            "Check for outdated packages",
            false,
        )]
    }
}
