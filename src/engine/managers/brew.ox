//! Homebrew package manager

import super.{Action, Manager, PackageManager}

/// Homebrew package manager
public struct BrewManager

extension BrewManager: PackageManager {
    fn updateActions(): [Action] {
        [Action.new(
            Manager.brew,
            "brew update",
            "Update Homebrew formulae",
            false
        )]
    }

    fn upgradeActions(): [Action] {
        [
            Action.new(
                Manager.brew,
                "brew upgrade",
                "Upgrade outdated formulae",
                false
            ),
            Action.new(
                Manager.brew,
                "brew cleanup",
                "Remove old versions",
                false
            ),
        ]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.brew,
            "brew outdated",
            "Check for outdated formulae",
            false
        )]
    }
}
