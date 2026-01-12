//! APT package manager (Debian/Ubuntu)

use super::{Action, Manager, PackageManager};

/// APT package manager
pub struct AptManager;

impl PackageManager for AptManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Apt,
            "apt update",
            "Update package lists",
            true,
        )]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![
            Action::new(Manager::Apt, "apt upgrade -y", "Upgrade all packages", true),
            Action::new(
                Manager::Apt,
                "apt autoremove -y",
                "Remove unused dependencies",
                true,
            ),
        ]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Apt,
            "apt list --upgradable",
            "Check for upgradable packages",
            false,
        )]
    }
}
