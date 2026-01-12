//! rbenv version manager

use super::{Action, ActionKind, Manager, PackageManager};

/// rbenv version manager
pub struct RbenvManager;

impl PackageManager for RbenvManager {
    fn name(&self) -> &'static str {
        "rbenv"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Rbenv,
            kind: ActionKind::Update,
            command: "rbenv rehash".to_string(),
            description: "Rehash rbenv shims".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![] // rbenv doesn't upgrade Ruby versions automatically
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
