//! Core types for the yup package manager

import std.collections.HashSet
import serde.{ Deserialize, Serialize }
import strum.{ AsRefStr, Display, EnumCount, EnumIter, EnumString }

/// Manager enum - represents each supported package or version manager.
@[derive(Debug , Clone , Copy , PartialEq , Eq , Hash , Serialize , Deserialize , AsRefStr , Display , EnumString , EnumIter , EnumCount ,)]
@[strum(ascii_case_insensitive)]
public enum Manager {
  case brew
  case port
  case apt
  case dnf
  case pacman
  case flatpak
  case snap
  case winget
  case choco
  case scoop
  case mas
  case softwareupdate
  case mise
  case conda
  case npm
  case pnpm
  case pipx
  case cargo
  case rustup
  case gem
}

/// A single discrete action to be performed by a package manager.
@[derive(Debug , Clone)]
public struct Action(manager: Manager, command: String, description: String, requiresPrivilege: bool)

extension Action {
  /// Create a new action
  public fn new(manager: Manager, command: impl Into<String>, description: impl Into<String>, requiresPrivilege: bool): Action {
    Action(manager: manager, command: command.into(), description: description.into(), requiresPrivilege: requiresPrivilege)
  }
}

/// Comprehensive report of the system scan results.
@[derive(Debug , Default)]
public struct ScanReport(availableManagers: HashSet<Manager>, actionableManagers: HashSet<Manager>)
