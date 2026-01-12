//! Windows Package Manager (winget)

use super::{Action, ActionKind, Manager, PackageManager};

/// Windows Package Manager
pub struct WingetManager;

impl PackageManager for WingetManager {
    fn name(&self) -> &'static str {
        "Windows Package Manager"
    }

    fn update_actions(&self) -> Vec<Action> {
        // winget upgrade does both update and upgrade
        vec![]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Winget,
            kind: ActionKind::Upgrade,
            command: "winget upgrade --all --accept-package-agreements --accept-source-agreements"
                .to_string(),
            description: "Upgrade all packages".to_string(),
            requires_privilege: false,
        }]
    }

    fn requires_privilege(&self) -> bool {
        false // winget runs as user, may prompt UAC
    }
}
