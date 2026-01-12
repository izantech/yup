//! Windows Package Manager (winget)

use super::{Action, Manager, PackageManager};

/// Windows Package Manager
pub struct WingetManager;

impl PackageManager for WingetManager {
    fn update_actions(&self) -> Vec<Action> {
        // winget upgrade does both update and upgrade
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Winget,
            "winget upgrade --all --silent --disable-interactivity --accept-package-agreements --accept-source-agreements",
            "Upgrade all packages",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Winget,
            "winget list --upgrade-available",
            "Check for available upgrades",
            false,
        )]
    }
}
