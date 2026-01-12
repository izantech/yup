//! pnpm package manager

use super::{Action, Manager, PackageManager};

/// pnpm package manager
pub struct PnpmManager;

impl PackageManager for PnpmManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Pnpm,
            "pnpm self-update",
            "Update pnpm itself",
            false,
        )]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Pnpm,
            "pnpm update -g",
            "Update global pnpm packages",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Pnpm,
            "pnpm outdated -g",
            "Check for outdated global pnpm packages",
            false,
        )]
    }
}
