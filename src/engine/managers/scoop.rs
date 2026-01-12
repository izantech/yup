//! Scoop package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Scoop package manager
pub struct ScoopManager;

impl PackageManager for ScoopManager {
    fn name(&self) -> &'static str {
        "Scoop"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Scoop,
            kind: ActionKind::Update,
            command: "scoop update".to_string(),
            description: "Update Scoop and manifests".to_string(),
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Scoop,
            kind: ActionKind::Upgrade,
            command: "scoop update *".to_string(),
            description: "Update all packages".to_string(),
        }]
    }

    fn requires_privilege(&self) -> bool {
        false // scoop runs as user
    }
}
