//! Chocolatey package manager

use super::{Action, Manager, PackageManager};

/// Chocolatey package manager
pub struct ChocoManager;

impl PackageManager for ChocoManager {
    fn update_actions(&self) -> Vec<Action> {
        // choco upgrade does both update and upgrade
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Choco,
            "choco upgrade all -y",
            "Upgrade all packages",
            true,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Choco,
            "choco outdated",
            "Check for outdated packages",
            false,
        )]
    }
}
