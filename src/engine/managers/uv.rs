//! uv package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// uv - fast Python package installer and resolver
pub struct UvManager;

impl PackageManager for UvManager {
    fn name(&self) -> &'static str {
        "uv"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Uv,
            kind: ActionKind::Update,
            command: "uv self update".to_string(),
            description: "Update uv itself".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // uv is per-project like poetry, no global packages to upgrade
        vec![]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
