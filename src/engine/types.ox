//! Core types for the yup package manager

import std.collections.HashSet
import serde.{ Deserialize, Serialize }
import strum.{ AsRefStr, Display, EnumCount, EnumIter, EnumString }

/// Manager enum - represents each supported package or version manager.
@[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
  AsRefStr,
  Display,
  EnumString,
  EnumIter,
  EnumCount,
)]
@[strum(ascii_case_insensitive)]
public enum Manager {
  Brew,
  Port,
  Apt,
  Dnf,
  Pacman,
  Flatpak,
  Snap,
  Winget,
  Choco,
  Scoop,
  Mas,
  SoftwareUpdate,
  Mise,
  Conda,
  Npm,
  Pnpm,
  Pipx,
  Cargo,
  Rustup,
  Gem,
}

/// A single discrete action to be performed by a package manager.
@[derive(Debug, Clone)]
public struct Action(manager: Manager, command: String, description: String, requiresPrivilege: bool)

/// Comprehensive report of the system scan results.
@[derive(Debug, Default)]
public struct ScanReport(availableManagers: HashSet<Manager>, actionableManagers: HashSet<Manager>)
