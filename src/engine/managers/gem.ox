//! gem package manager (RubyGems)

import super.{Action, Manager, PackageManager}

/// RubyGems package manager
public struct GemManager

extension GemManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Gem,
            "gem update --system",
            "Update RubyGems itself",
            false
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Gem,
            "gem update",
            "Update all installed gems",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![Action.new(
            Manager.Gem,
            "gem outdated",
            "Check for outdated Ruby gems",
            false
        )]
    }
}
