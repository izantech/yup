//! Homebrew package manager

use super::{Action, Manager, PackageManager};

/// Homebrew package manager
pub struct BrewManager;

impl PackageManager for BrewManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Brew,
            "brew update",
            "Update Homebrew formulae",
            false,
        )]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![
            Action::new(
                Manager::Brew,
                "brew upgrade",
                "Upgrade Homebrew packages",
                false,
            ),
            Action::new(
                Manager::Brew,
                "brew upgrade --cask",
                "Upgrade Homebrew casks",
                false,
            ),
        ]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![
            Action::new(
                Manager::Brew,
                "brew outdated",
                "Check for outdated Homebrew formulae",
                false,
            ),
            Action::new(
                Manager::Brew,
                "brew outdated --cask",
                "Check for outdated Homebrew casks",
                false,
            ),
        ]
    }
}
