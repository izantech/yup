//! macOS Software Update

use super::{Action, Manager, PackageManager};

/// macOS Software Update manager
pub struct SoftwareUpdateManager;

impl PackageManager for SoftwareUpdateManager {
    fn update_actions(&self) -> Vec<Action> {
        // softwareupdate list is automatic
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::SoftwareUpdate,
            "softwareupdate -ia",
            "Install all macOS updates (may require restart)",
            true,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::SoftwareUpdate,
            "softwareupdate -l",
            "Check for available macOS updates",
            false,
        )]
    }
}
