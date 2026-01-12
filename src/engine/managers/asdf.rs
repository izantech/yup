//! asdf version manager

use super::{Action, ActionKind, Manager, PackageManager};

/// asdf version manager
pub struct AsdfManager;

impl PackageManager for AsdfManager {
    fn name(&self) -> &'static str {
        "asdf"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Asdf,
            kind: ActionKind::Update,
            command: "asdf plugin update --all".to_string(),
            description: "Update all asdf plugins".to_string(),
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![] // asdf doesn't upgrade installed versions
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
