//! Volta (JavaScript tool manager)

use super::{Action, ActionKind, Manager, PackageManager};

/// Volta version manager
pub struct VoltaManager;

impl PackageManager for VoltaManager {
    fn name(&self) -> &'static str {
        "volta"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Volta,
            kind: ActionKind::Update,
            command: "volta setup".to_string(),
            description: "Update Volta".to_string(),
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![] // volta doesn't upgrade tools automatically
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
