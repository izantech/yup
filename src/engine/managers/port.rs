//! MacPorts package manager

use super::{Action, Manager, PackageManager};

/// MacPorts package manager
pub struct PortManager;

impl PackageManager for PortManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Port,
            "port selfupdate",
            "Update MacPorts and port definitions",
            true,
        )]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Port,
            "port upgrade outdated",
            "Upgrade outdated ports",
            true,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Port,
            "port outdated",
            "Check for outdated ports",
            false,
        )]
    }
}
