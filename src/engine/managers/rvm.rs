//! RVM (Ruby Version Manager)

use super::{Action, ActionKind, Manager, PackageManager};

/// RVM version manager
pub struct RvmManager;

impl PackageManager for RvmManager {
    fn name(&self) -> &'static str {
        "RVM"
    }

    fn update_actions(&self) -> Vec<Action> {
        // RVM is a shell function, needs bash wrapper to source it
        vec![Action {
            manager: Manager::Rvm,
            kind: ActionKind::Update,
            command: r#"bash -c 'source "${rvm_path:-$HOME/.rvm}/scripts/rvm" && rvm get stable'"#
                .to_string(),
            description: "Update RVM to stable".to_string(),
            requires_privilege: false,
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![] // RVM doesn't upgrade installed Ruby versions
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
