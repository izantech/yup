//! pnpm package manager

import super.{Action, Manager, PackageManager}

/// pnpm package manager
public struct PnpmManager

extension PnpmManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Pnpm,
            "pnpm self-update",
            "Update pnpm itself",
            false
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Pnpm,
            "pnpm update -g",
            "Update global pnpm packages",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Pnpm,
            "pnpm outdated -g",
            "Check for outdated global pnpm packages",
            false
        )]
    }
}
