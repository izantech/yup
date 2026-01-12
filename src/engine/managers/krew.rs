//! krew - kubectl plugin manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Krew - kubectl plugin manager
pub struct KrewManager;

impl PackageManager for KrewManager {
    fn name(&self) -> &'static str {
        "krew"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Krew,
            kind: ActionKind::Update,
            command: "kubectl krew update".to_string(),
            description: "Update krew plugin index".to_string(),
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Krew,
            kind: ActionKind::Upgrade,
            command: "kubectl krew upgrade".to_string(),
            description: "Upgrade all kubectl plugins".to_string(),
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
