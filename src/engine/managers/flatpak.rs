//! Flatpak package manager

use super::{Action, Manager, PackageManager};

/// Flatpak package manager
pub struct FlatpakManager;

impl PackageManager for FlatpakManager {
    fn update_actions(&self) -> Vec<Action> {
        // flatpak update does both update and upgrade
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Flatpak,
            "flatpak update -y",
            "Update all Flatpak applications",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Flatpak,
            "flatpak remote-ls --updates",
            "Check for available updates",
            false,
        )]
    }
}
