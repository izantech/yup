//! pnpm package manager

import super.{Action, Manager, PackageManager}

/// pnpm package manager
public struct PnpmManager

extension PnpmManager: PackageManager {
    fn updateActions(): [Action] {
        [Action.new(
            Manager.pnpm,
            "pnpm self-update",
            "Update pnpm itself",
            false
        )]
    }

    fn upgradeActions(): [Action] {
        [Action.new(
            Manager.pnpm,
            "pnpm update -g",
            "Update global pnpm packages",
            false
        )]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.pnpm,
            "pnpm outdated -g",
            "Check for outdated global pnpm packages",
            false
        )]
    }
}
