//! pipx package manager

import super.{Action, Manager, PackageManager}

/// pipx - install and run Python applications in isolated environments
public struct PipxManager

extension PipxManager: PackageManager {
    fn updateActions(): [Action] {
        // pipx is typically managed by pip or brew, no self-update command
        []
    }

    fn upgradeActions(): [Action] {
        // pipx upgrade-all is safe because each app is in its own isolated venv
        [Action.new(
            Manager.pipx,
            "pipx upgrade-all",
            "Upgrade all pipx-installed packages",
            false
        )]
    }

    fn checkActions(): [Action] {
        // pipx doesn't provide a read-only "outdated" or dry-run upgrade command.
        []
    }
}
