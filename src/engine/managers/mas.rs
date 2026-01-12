//! Mac App Store CLI (mas)

use super::{Action, Manager, PackageManager};

/// Mac App Store CLI manager
pub struct MasManager;

impl PackageManager for MasManager {
    fn update_actions(&self) -> Vec<Action> {
        // mas has no separate update step
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Mas,
            "mas upgrade",
            "Upgrade Mac App Store apps",
            true,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Mas,
            "mas outdated",
            "Check for outdated Mac App Store apps",
            false,
        )]
    }
}
