//! pipx package manager

use super::{Action, Manager, PackageManager};

/// pipx - install and run Python applications in isolated environments
pub struct PipxManager;

impl PackageManager for PipxManager {
    fn update_actions(&self) -> Vec<Action> {
        // pipx is typically managed by pip or brew, no self-update command
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // pipx upgrade-all is safe because each app is in its own isolated venv
        vec![Action::new(
            Manager::Pipx,
            "pipx upgrade-all",
            "Upgrade all pipx-installed packages",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Pipx,
            "pipx list --outdated",
            "Check for outdated pipx packages",
            false,
        )]
    }
}
