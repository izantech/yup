//! go language runtime

use super::{Action, PackageManager};

/// Go language runtime
///
/// Note: Go intentionally has no actions - the runtime is managed by system
/// package managers (brew, apt, etc.). `go get -u` is per-module and not
/// suitable for global use.
pub struct GoManager;

impl PackageManager for GoManager {
    fn name(&self) -> &'static str {
        "go"
    }

    fn update_actions(&self) -> Vec<Action> {
        // Go runtime is updated via brew/system manager
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // go get -u is per-module, not suitable for global use
        vec![]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
