//! mise version manager (formerly rtx)

import super.{Action, Manager, PackageManager}

/// mise version manager - a modern alternative to asdf
public struct MiseManager

extension MiseManager: PackageManager {
    fn updateActions(): [Action] {
        [
            Action.new(
                Manager.mise,
                "mise self-update",
                "Update mise itself",
                false
            ),
            Action.new(
                Manager.mise,
                "mise plugins update",
                "Update all mise plugins",
                false
            ),
        ]
    }

    fn upgradeActions(): [Action] {
        [Action.new(
            Manager.mise,
            "mise upgrade",
            "Upgrade all mise-managed tools",
            false
        )]
    }

    fn checkActions(): [Action] {
        [Action.new(
            Manager.mise,
            "mise outdated",
            "Check for outdated mise-managed tools",
            false
        )]
    }
}
