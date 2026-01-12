//! mise version manager (formerly rtx)

use super::{Action, ActionKind, Manager, PackageManager};

/// mise version manager - a modern alternative to asdf
pub struct MiseManager;

impl PackageManager for MiseManager {
    fn name(&self) -> &'static str {
        "mise"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![
            Action {
                manager: Manager::Mise,
                kind: ActionKind::Update,
                command: "mise self-update".to_string(),
                description: "Update mise itself".to_string(),
                requires_privilege: false,
            },
            Action {
                manager: Manager::Mise,
                kind: ActionKind::Update,
                command: "mise plugins update".to_string(),
                description: "Update all mise plugins".to_string(),
                requires_privilege: false,
            },
        ]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Mise,
            kind: ActionKind::Upgrade,
            command: "mise upgrade".to_string(),
            description: "Upgrade all mise-managed tools".to_string(),
            requires_privilege: false,
        }]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Mise,
            kind: ActionKind::Check,
            command: "mise outdated".to_string(),
            description: "Check for outdated mise-managed tools".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
