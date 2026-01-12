//! gem package manager (RubyGems)

use super::{Action, Manager, PackageManager};

/// RubyGems package manager
pub struct GemManager;

impl PackageManager for GemManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Gem,
            "gem update --system",
            "Update RubyGems itself",
            false,
        )]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Gem,
            "gem update",
            "Update all installed gems",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Gem,
            "gem outdated",
            "Check for outdated Ruby gems",
            false,
        )]
    }
}
