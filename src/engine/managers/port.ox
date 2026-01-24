//! MacPorts package manager

import super.{Action, Manager, PackageManager}

/// MacPorts package manager
public struct PortManager

extension PortManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Port,
            "port selfupdate",
            "Update MacPorts and port definitions",
            true
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Port,
            "port upgrade outdated",
            "Upgrade outdated ports",
            true
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Port,
            "port outdated",
            "Check for outdated ports",
            false
        )]
    }
}
