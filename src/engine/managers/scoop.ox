//! Scoop package manager

import super.{Action, Manager, PackageManager}

/// Scoop package manager
public struct ScoopManager

extension ScoopManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Scoop,
            "scoop update",
            "Update Scoop and manifests",
            false
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Scoop,
            "scoop update *",
            "Update all packages",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Scoop,
            "scoop status",
            "Check for outdated packages",
            false
        )]
    }
}
