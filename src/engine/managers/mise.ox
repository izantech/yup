//! mise version manager (formerly rtx)

import super.{Action, Manager, PackageManager}

/// mise version manager - a modern alternative to asdf
public struct MiseManager

extension MiseManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![
            Action.new(
                Manager.Mise,
                "mise self-update",
                "Update mise itself",
                false
            ),
            Action.new(
                Manager.Mise,
                "mise plugins update",
                "Update all mise plugins",
                false
            ),
        ]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Mise,
            "mise upgrade",
            "Upgrade all mise-managed tools",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Mise,
            "mise outdated",
            "Check for outdated mise-managed tools",
            false
        )]
    }
}
