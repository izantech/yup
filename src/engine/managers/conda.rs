//! conda package/environment manager

use super::{Action, Manager, PackageManager};

/// conda package manager
pub struct CondaManager;

impl PackageManager for CondaManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Conda,
            "conda update -n base conda -y",
            "Update conda itself",
            false,
        )]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        // Upgrading all base packages can break environments
        vec![Action::new(
            Manager::Conda,
            "conda update -n base --all -y",
            "Update all packages in base environment",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Conda,
            "conda list --outdated",
            "Check for outdated packages",
            false,
        )]
    }
}
