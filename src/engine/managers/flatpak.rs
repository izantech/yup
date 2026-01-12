//! Flatpak package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Flatpak package manager
pub struct FlatpakManager;

impl PackageManager for FlatpakManager {
    fn name(&self) -> &'static str {
        "Flatpak"
    }

    fn update_actions(&self) -> Vec<Action> {
        // flatpak update does both update and upgrade
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Flatpak,
            kind: ActionKind::Upgrade,
            command: "flatpak update -y".to_string(),
            description: "Update all Flatpak applications".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        false // flatpak runs as user by default
    }
}
