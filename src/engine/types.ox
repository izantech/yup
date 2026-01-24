import std.collections.HashSet

import serde.{Deserialize, Serialize}
import strum.{AsRefStr, Display, EnumCount, EnumIter, EnumString}

/// Manager enum - represents each supported package or version manager.
@[derive(
    Debug ,
    Clone ,
    Copy ,
    PartialEq ,
    Eq ,
    Hash ,
    Serialize ,
    Deserialize ,
    AsRefStr ,
    Display ,
    EnumString ,
    EnumIter ,
    EnumCount
)]
@[strum(ascii_case_insensitive)]
@[serde(rename_all = "lowercase")]
public enum Manager {
    case Brew
    case Port
    case Apt
    case Dnf
    case Pacman
    case Flatpak
    case Snap
    case Winget
    case Choco
    case Scoop
    case Mas
    case SoftwareUpdate
    case Mise
    case Conda
    case Npm
    case Pnpm
    case Pipx
    case Cargo
    case Rustup
    case Gem
}

/// A single discrete action to be performed by a package manager.
@[derive(Debug , Clone)]
public struct Action(
    manager: Manager,
    command: String,
    description: String,
    requires_privilege: bool
)

/// Comprehensive report of the system scan results.
@[derive(Debug , Default)]
public struct ScanReport(
    available_managers: HashSet<Manager>,
    actionable_managers: HashSet<Manager>
)

extension Action {
    /// Create a new action
    public fn new(
        manager: Manager,
        command: impl Into<String>,
        description: impl Into<String>,
        requires_privilege: bool
    ): Action {
        Action(
            manager,
            command.into(),
            description.into(),
            requires_privilege
        )
    }
}
