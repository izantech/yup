//! helm package manager (Kubernetes)

use super::{Action, ActionKind, Manager, PackageManager};

/// Helm - Kubernetes package manager
pub struct HelmManager;

impl PackageManager for HelmManager {
    fn name(&self) -> &'static str {
        "helm"
    }

    fn update_actions(&self) -> Vec<Action> {
        vec![Action {
            manager: Manager::Helm,
            kind: ActionKind::Update,
            command: "helm repo update".to_string(),
            description: "Update Helm repository metadata".to_string(),
        }]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Helm charts are per-cluster deployments, not global packages
        vec![]
    }

    fn requires_privilege(&self) -> bool {
        false
    }
}
