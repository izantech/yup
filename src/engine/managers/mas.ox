//! Mac App Store CLI (mas)

import super.{Action, Manager, PackageManager}

/// Mac App Store CLI manager
public struct MasManager

extension MasManager: PackageManager {
    fn update_actions(): Vec<Action> {
        // mas has no separate update step
        vec![]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Mas,
            "mas upgrade",
            "Upgrade Mac App Store apps",
            true
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Mas,
            "mas outdated",
            "Check for outdated Mac App Store apps",
            false
        )]
    }
}
