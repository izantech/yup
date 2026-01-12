//! nvm (Node Version Manager)

use super::{Action, ActionKind, Manager, PackageManager};

/// nvm version manager
pub struct NvmManager;

impl PackageManager for NvmManager {
    fn name(&self) -> &'static str {
        "nvm"
    }

    fn update_actions(&self) -> Vec<Action> {
        // nvm is a shell function, requires bash wrapper to source
        vec![Action {
            manager: Manager::Nvm,
            kind: ActionKind::Update,
            command: r#"bash -c 'export NVM_DIR="${NVM_DIR:-$HOME/.nvm}" && . "$NVM_DIR/nvm.sh" && nvm install-latest-npm'"#.to_string(),
            description: "Update npm to latest for current Node".to_string(),
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![] // nvm doesn't upgrade Node versions automatically
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
