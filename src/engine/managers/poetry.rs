//! poetry package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Poetry - Python dependency management and packaging
pub struct PoetryManager;

impl PackageManager for PoetryManager {
    fn name(&self) -> &'static str {
        "poetry"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Poetry,
            kind: ActionKind::Update,
            command: "poetry self update".to_string(),
            description: "Update Poetry itself".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Poetry is per-project, no global packages to upgrade
        vec![]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
