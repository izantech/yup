//! npm package manager

use super::{Action, Manager, PackageManager};

/// npm package manager (comes with Node.js)
pub struct NpmManager;

impl PackageManager for NpmManager {
    fn update_actions(&self) -> Vec<Action> {
        // npm is updated with Node.js itself, no separate update needed
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Npm,
            "npm update -g",
            "Update global npm packages",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Npm,
            "npm outdated -g",
            "Check for outdated global npm packages",
            false,
        )]
    }
}
