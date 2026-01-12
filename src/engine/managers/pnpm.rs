//! pnpm package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// pnpm package manager
pub struct PnpmManager;

impl PackageManager for PnpmManager {
    fn name(&self) -> &'static str {
        "pnpm"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Pnpm,
            kind: ActionKind::Update,
            command: "pnpm self-update".to_string(),
            description: "Update pnpm itself".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Pnpm,
            kind: ActionKind::Upgrade,
            command: "pnpm update -g".to_string(),
            description: "Update global pnpm packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Pnpm,
            kind: ActionKind::Check,
            command: "pnpm outdated -g".to_string(),
            description: "Check for outdated global pnpm packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
