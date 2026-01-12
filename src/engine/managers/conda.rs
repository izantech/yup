//! conda package/environment manager

use super::{Action, ActionKind, Manager, PackageManager};

/// conda package manager
pub struct CondaManager;

impl PackageManager for CondaManager {
    fn name(&self) -> &'static str {
        "conda"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Conda,
            kind: ActionKind::Update,
            command: "conda update -n base conda -y".to_string(),
            description: "Update conda itself".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Upgrading all base packages can break environments
        vec![Action {
            manager: Manager::Conda,
            kind: ActionKind::Upgrade,
            command: "conda update -n base --all -y".to_string(),
            description: "Update all packages in base environment".to_string(),
            requires_privilege: false,
        }]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Conda,
            kind: ActionKind::Check,
            command: "conda list --outdated".to_string(),
            description: "Check for outdated packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
