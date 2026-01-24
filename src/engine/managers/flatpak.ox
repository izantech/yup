//! Flatpak package manager

import super.{Action, Manager, PackageManager}

/// Flatpak package manager
public struct FlatpakManager

extension FlatpakManager: PackageManager {
    fn update_actions(): Vec<Action> {
        // flatpak update does both update and upgrade
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Flatpak,
            "flatpak update -y",
            "Update all Flatpak applications",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Flatpak,
            "flatpak remote-ls --updates",
            "Check for available updates",
            false
        )]
    }
}
