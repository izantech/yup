//! npm package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// npm package manager (comes with Node.js)
pub struct NpmManager;

impl PackageManager for NpmManager {
    fn name(&self) -> &'static str {
        "npm"
    }

    fn update_actions(&self) -> Vec<Action> {
        // npm is updated with Node.js itself, no separate update needed
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Npm,
            kind: ActionKind::Upgrade,
            command: "npm update -g".to_string(),
            description: "Update global npm packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Npm,
            kind: ActionKind::Check,
            command: "npm outdated -g".to_string(),
            description: "Check for outdated global npm packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
