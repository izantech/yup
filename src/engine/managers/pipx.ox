//! pipx package manager

import super.{Action, Manager, PackageManager}

/// pipx - install and run Python applications in isolated environments
public struct PipxManager

extension PipxManager: PackageManager {
    fn update_actions(): Vec<Action> {
        // pipx is typically managed by pip or brew, no self-update command
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        // pipx upgrade-all is safe because each app is in its own isolated venv
        vec![Action.new(
            Manager.Pipx,
            "pipx upgrade-all",
            "Upgrade all pipx-installed packages",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        // pipx doesn't provide a read-only "outdated" or dry-run upgrade command.
        vec![]
    }
}
