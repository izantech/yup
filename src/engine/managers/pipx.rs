//! pipx package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// pipx - install and run Python applications in isolated environments
pub struct PipxManager;

impl PackageManager for PipxManager {
    fn name(&self) -> &'static str {
        "pipx"
    }

    fn update_actions(&self) -> Vec<Action> {
        // pipx is typically managed by pip or brew, no self-update command
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // pipx upgrade-all is safe because each app is in its own isolated venv
        vec![Action {
            manager: Manager::Pipx,
            kind: ActionKind::Upgrade,
            command: "pipx upgrade-all".to_string(),
            description: "Upgrade all pipx-installed packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Pipx,
            kind: ActionKind::Check,
            command: "pipx list --outdated".to_string(),
            description: "Check for outdated pipx packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
