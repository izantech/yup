//! pyenv version manager

use std::path::Path;

use super::{Action, ActionKind, Manager, PackageManager};

/// pyenv version manager
pub struct PyenvManager;

impl PackageManager for PyenvManager {
    fn name(&self) -> &'static str {
        "pyenv"
    }

    fn update_actions(&self) -> Vec<Action> {
        // Only return action if pyenv-update plugin is installed
        if pyenv_update_available() {
            vec![Action {
                manager: Manager::Pyenv,
                kind: ActionKind::Update,
                command: "pyenv update".to_string(),
                description: "Update pyenv and plugins".to_string(),
                requires_privilege: false,
            }]
        } else {
            vec![] // No pyenv-update plugin, can't self-update
        }
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![] // pyenv doesn't upgrade installed Python versions
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}

/// Check if pyenv-update plugin is installed
fn pyenv_update_available() -> bool {
    // Check PYENV_ROOT first, then default ~/.pyenv
    if let Ok(root) = std::env::var("PYENV_ROOT") {
        if Path::new(&format!("{}/plugins/pyenv-update", root)).exists() {
            return true;
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        if Path::new(&format!("{}/.pyenv/plugins/pyenv-update", home)).exists() {
            return true;
        }
    }
    false
}
