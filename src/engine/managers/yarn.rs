//! yarn package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Yarn package manager (modern Yarn Berry via corepack)
pub struct YarnManager;

impl PackageManager for YarnManager {
    fn name(&self) -> &'static str {
        "yarn"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Yarn,
            kind: ActionKind::Update,
            command: "yarn set version stable".to_string(),
            description: "Update Yarn to stable version".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Modern Yarn Berry is project-local, not global
        // Yarn 1.x global packages are deprecated
        vec![]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
