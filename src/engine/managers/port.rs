//! MacPorts package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// MacPorts package manager
pub struct PortManager;

impl PackageManager for PortManager {
    fn name(&self) -> &'static str {
        "MacPorts"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Port,
            kind: ActionKind::Update,
            command: "port selfupdate".to_string(),
            description: "Update MacPorts and port definitions".to_string(),
            requires_privilege: true,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Port,
            kind: ActionKind::Upgrade,
            command: "port upgrade outdated".to_string(),
            description: "Upgrade outdated ports".to_string(),
            requires_privilege: true,
        }]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Port,
            kind: ActionKind::Check,
            command: "port outdated".to_string(),
            description: "Check for outdated ports".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        true
    }
}
