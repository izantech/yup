//! Snap package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Snap package manager
pub struct SnapManager;

impl PackageManager for SnapManager {
    fn name(&self) -> &'static str {
        "Snap"
    }

    fn update_actions(&self) -> Vec<Action> {
        // snap refresh does both update and upgrade
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Snap,
            kind: ActionKind::Upgrade,
            command: "snap refresh".to_string(),
            description: "Refresh all Snap packages".to_string(),
            requires_privilege: true,
        }]
    }

    fn requires_privilege(&self) -> bool {
        true // snap refresh needs sudo
    }
}
