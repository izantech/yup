//! fnm (Fast Node Manager)

use super::{Action, PackageManager};

/// fnm version manager
pub struct FnmManager;

impl PackageManager for FnmManager {
    fn name(&self) -> &'static str {
        "fnm"
    }

    fn update_actions(&self) -> Vec<Action> {
        // fnm self-update only works if installed via official installer
        // If installed via brew/cargo, it updates through those managers
        // Return empty since we can't reliably determine installation method
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![] // fnm doesn't upgrade Node versions automatically
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
