//! SDKMAN! - Software Development Kit Manager (Java ecosystem)

use super::{Action, ActionKind, Manager, PackageManager};

/// SDKMAN! - manages Java, Kotlin, Scala, Groovy SDKs
///
/// SDKMAN is a shell function like RVM, requires bash wrapper to source it.
pub struct SdkmanManager;

impl PackageManager for SdkmanManager {
    fn name(&self) -> &'static str {
        "sdkman"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![
            Action {
                manager: Manager::Sdkman,
                kind: ActionKind::Update,
                command: r#"bash -c 'source "${SDKMAN_DIR:-$HOME/.sdkman}/bin/sdkman-init.sh" && sdk selfupdate'"#
                    .to_string(),
                description: "Update SDKMAN itself".to_string(),
                requires_privilege: false,
            },
            Action {
                manager: Manager::Sdkman,
                kind: ActionKind::Update,
                command: r#"bash -c 'source "${SDKMAN_DIR:-$HOME/.sdkman}/bin/sdkman-init.sh" && sdk update'"#.to_string(),
                description: "Update SDKMAN candidate list".to_string(),
                requires_privilege: false,
            },
        ]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // SDKMAN doesn't bulk-upgrade installed versions
        // Users explicitly choose which SDK versions to install
        vec![]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
