//! bun runtime and package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Bun runtime and package manager
pub struct BunManager;

impl PackageManager for BunManager {
    fn name(&self) -> &'static str {
        "bun"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Bun,
            kind: ActionKind::Update,
            command: "bun upgrade".to_string(),
            description: "Upgrade Bun to latest version".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Bun global packages are rare, skip for now
        vec![]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
