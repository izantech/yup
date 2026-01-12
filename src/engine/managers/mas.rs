//! Mac App Store CLI (mas)

use super::{Action, ActionKind, Manager, PackageManager};

/// Mac App Store CLI manager
pub struct MasManager;

impl PackageManager for MasManager {
    fn name(&self) -> &'static str {
        "Mac App Store"
    }

    fn update_actions(&self) -> Vec<Action> {
        // mas has no separate update step
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Mas,
            kind: ActionKind::Upgrade,
            command: "mas upgrade".to_string(),
            description: "Upgrade Mac App Store apps".to_string(),
        }]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Mas,
            kind: ActionKind::Check,
            command: "mas outdated".to_string(),
            description: "Check for outdated Mac App Store apps".to_string(),
        }]
    }

    fn requires_privilege(&self) -> bool {
        // mas 4.0+ requires sudo on macOS 15+
        true
    }
}
