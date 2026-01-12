//! pip package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// pip package manager (Python's built-in package manager)
pub struct PipManager;

impl PackageManager for PipManager {
    fn name(&self) -> &'static str {
        "pip"
    }

    fn update_actions(&self) -> Vec<Action> {
        // Self-update pip itself (safe)
        vec![Action {
            manager: Manager::Pip,
            kind: ActionKind::Update,
            command: "pip install --upgrade pip".to_string(),
            description: "Upgrade pip itself".to_string(),
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // DO NOT implement mass pip upgrade - it breaks systems
        // Per-venv is the correct pattern for Python packages
        vec![]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Pip,
            kind: ActionKind::Check,
            command: "pip list --outdated".to_string(),
            description: "Check for outdated pip packages".to_string(),
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
