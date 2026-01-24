//! npm package manager

import super.{Action, Manager, PackageManager}

/// npm package manager
public struct NpmManager

extension NpmManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Npm,
            "npm install -g npm",
            "Update npm itself",
            false
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Npm,
            "npm update -g",
            "Update global npm packages",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Npm,
            "npm outdated -g",
            "Check for outdated global npm packages",
            false
        )]
    }
}
