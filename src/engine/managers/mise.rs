//! mise version manager (formerly rtx)

use super::{Action, Manager, PackageManager};

/// mise version manager - a modern alternative to asdf
pub struct MiseManager;

impl PackageManager for MiseManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![
            Action::new(
                Manager::Mise,
                "mise self-update",
                "Update mise itself",
                false,
            ),
            Action::new(
                Manager::Mise,
                "mise plugins update",
                "Update all mise plugins",
                false,
            ),
        ]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Mise,
            "mise upgrade",
            "Upgrade all mise-managed tools",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Mise,
            "mise outdated",
            "Check for outdated mise-managed tools",
            false,
        )]
    }
}
