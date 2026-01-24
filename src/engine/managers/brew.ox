//! Homebrew package manager

import super.{Action, Manager, PackageManager}

/// Homebrew package manager
public struct BrewManager

extension BrewManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Brew,
            "brew update",
            "Update Homebrew formulae",
            false
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![
            Action.new(
                Manager.Brew,
                "brew upgrade",
                "Upgrade outdated formulae",
                false
            ),
            Action.new(
                Manager.Brew,
                "brew cleanup",
                "Remove old versions",
                false
            ),
        ]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Brew,
            "brew outdated",
            "Check for outdated formulae",
            false
        )]
    }
}
