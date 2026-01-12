//! DNF package manager (Fedora/RHEL)

use super::{Action, Manager, PackageManager};

/// DNF package manager
pub struct DnfManager;

impl PackageManager for DnfManager {
    fn update_actions(&self) -> Vec<Action> {
        // DNF auto-syncs during upgrade, no separate update needed
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![
            Action::new(Manager::Dnf, "dnf upgrade -y", "Upgrade all packages", true),
            Action::new(
                Manager::Dnf,
                "dnf autoremove -y",
                "Remove unused dependencies",
                true,
            ),
        ]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Dnf,
            "dnf check-update",
            "Check for available updates",
            false,
        )]
    }
}
