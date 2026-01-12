//! gem package manager (RubyGems)

use super::{Action, ActionKind, Manager, PackageManager};

/// RubyGems package manager
pub struct GemManager;

impl PackageManager for GemManager {
    fn name(&self) -> &'static str {
        "gem"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Gem,
            kind: ActionKind::Update,
            command: "gem update --system".to_string(),
            description: "Update RubyGems itself".to_string(),
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Gem,
            kind: ActionKind::Upgrade,
            command: "gem update".to_string(),
            description: "Update all installed gems".to_string(),
        }]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Gem,
            kind: ActionKind::Check,
            command: "gem outdated".to_string(),
            description: "Check for outdated Ruby gems".to_string(),
        }]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
