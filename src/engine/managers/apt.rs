//! APT package manager (Debian/Ubuntu)

use super::{Action, ActionKind, Manager, PackageManager};

/// APT package manager
pub struct AptManager;

impl PackageManager for AptManager {
    fn name(&self) -> &'static str {
        "APT"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Apt,
            kind: ActionKind::Update,
            command: "apt update".to_string(),
            description: "Update package lists".to_string(),
            requires_privilege: true,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![
            Action {
                manager: Manager::Apt,
                kind: ActionKind::Upgrade,
                command: "apt upgrade -y".to_string(),
                description: "Upgrade all packages".to_string(),
                requires_privilege: true,
            },
            Action {
                manager: Manager::Apt,
                kind: ActionKind::Cleanup,
                command: "apt autoremove -y".to_string(),
                description: "Remove unused dependencies".to_string(),
                requires_privilege: true,
            },
        ]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Apt,
            kind: ActionKind::Check,
            command: "apt list --upgradable".to_string(),
            description: "Check for upgradable packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        true
    }
}
