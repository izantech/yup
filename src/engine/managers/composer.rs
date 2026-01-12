//! composer package manager (PHP)

use super::{Action, ActionKind, Manager, PackageManager};

/// Composer - PHP dependency manager
pub struct ComposerManager;

impl PackageManager for ComposerManager {
    fn name(&self) -> &'static str {
        "composer"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Composer,
            kind: ActionKind::Update,
            command: "composer self-update".to_string(),
            description: "Update Composer itself".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Composer,
            kind: ActionKind::Upgrade,
            command: "composer global update".to_string(),
            description: "Update global Composer packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
