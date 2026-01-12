//! DNF package manager (Fedora/RHEL)

use super::{Action, ActionKind, Manager, PackageManager};

/// DNF package manager
pub struct DnfManager;

impl PackageManager for DnfManager {
    fn name(&self) -> &'static str {
        "DNF"
    }

    fn update_actions(&self) -> Vec<Action> {
        // DNF auto-syncs during upgrade, no separate update needed
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![
            Action {
                manager: Manager::Dnf,
                kind: ActionKind::Upgrade,
                command: "dnf upgrade -y".to_string(),
                description: "Upgrade all packages".to_string(),
                requires_privilege: true,
            },
            Action {
                manager: Manager::Dnf,
                kind: ActionKind::Cleanup,
                command: "dnf autoremove -y".to_string(),
                description: "Remove unused dependencies".to_string(),
                requires_privilege: true,
            },
        ]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Dnf,
            kind: ActionKind::Check,
            command: "dnf check-update".to_string(),
            description: "Check for available updates".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        true
    }
}
