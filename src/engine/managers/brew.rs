//! Homebrew package manager

use super::{Action, ActionKind, Manager, PackageManager};

/// Homebrew package manager
pub struct BrewManager;

impl PackageManager for BrewManager {
    fn name(&self) -> &'static str {
        "Homebrew"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Brew,
            kind: ActionKind::Update,
            command: "brew update".to_string(),
            description: "Update Homebrew formulae".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![
            Action {
                manager: Manager::Brew,
                kind: ActionKind::Upgrade,
                command: "brew upgrade".to_string(),
                description: "Upgrade Homebrew packages".to_string(),
                requires_privilege: false,
            },
            Action {
                manager: Manager::Brew,
                kind: ActionKind::Upgrade,
                command: "brew upgrade --cask".to_string(),
                description: "Upgrade Homebrew casks".to_string(),
                requires_privilege: false,
            },
        ]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![
            Action {
                manager: Manager::Brew,
                kind: ActionKind::Check,
                command: "brew outdated".to_string(),
                description: "Check for outdated Homebrew formulae".to_string(),
                requires_privilege: false,
            },
            Action {
                manager: Manager::Brew,
                kind: ActionKind::Check,
                command: "brew outdated --cask".to_string(),
                description: "Check for outdated Homebrew casks".to_string(),
                requires_privilege: false,
            },
        ]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
