//! Flatpak package manager

import super.{Action, Manager, PackageManager}

/// Flatpak package manager
public struct FlatpakManager

extension FlatpakManager: PackageManager {
    fn updateActions(): [Action] {
        // flatpak update does both update and upgrade
        []
    }

    fn upgradeActions(): [Action] {
        [Action.new(
            Manager.flatpak,
            "flatpak update -y",
            "Update all Flatpak applications",
            false
        )]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.flatpak,
            "flatpak remote-ls --updates",
            "Check for available updates",
            false
        )]
    }
}
