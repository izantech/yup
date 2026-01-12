//! Chocolatey package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Chocolatey package manager
pub struct ChocoManager;

impl PackageManager for ChocoManager {
    fn name(&self) -> &'static str {
        "Chocolatey"
    }

    fn update_actions(&self) -> Vec<Action> {
        // choco upgrade does both update and upgrade
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Choco,
            kind: ActionKind::Upgrade,
            command: "choco upgrade all -y".to_string(),
            description: "Upgrade all packages".to_string(),
        }]
    }

    fn requires_privilege(&self) -> bool {
        true // choco needs admin
    }
}
