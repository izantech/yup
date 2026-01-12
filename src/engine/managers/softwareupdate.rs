//! macOS Software Update

use super::{Action, ActionKind, Manager, PackageManager};

/// macOS Software Update manager
pub struct SoftwareUpdateManager;

impl PackageManager for SoftwareUpdateManager {
    fn name(&self) -> &'static str {
        "macOS Software Update"
    }

    fn update_actions(&self) -> Vec<Action> {
        // softwareupdate list is automatic
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::SoftwareUpdate,
            kind: ActionKind::Upgrade,
            command: "softwareupdate -ia".to_string(),
            description: "Install all macOS updates (may require restart)".to_string(),
            requires_privilege: true,
        }]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::SoftwareUpdate,
            kind: ActionKind::Check,
            command: "softwareupdate -l".to_string(),
            description: "Check for available macOS updates".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        true
    }
}
