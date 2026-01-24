//! Package manager implementations

import super.types.{ Action, Manager }

/// Trait defining the interface for package manager implementations.
public trait PackageManager {
  fn updateActions(): [Action]
  fn upgradeActions(): [Action]
  fn checkActions(): [Action]
}

// Cross-platform managers
external module conda
external module mise
external module cargo
external module gem
external module npm
external module pipx
external module pnpm
external module rustup

// macOS managers
external module brew
external module mas
external module port
external module softwareupdate

// Linux managers
external module apt
external module dnf
external module flatpak
external module pacman
external module snap

// Windows managers
external module choco
external module scoop
external module winget

// Re-exports for cross-platform managers
public import conda.CondaManager
public import mise.MiseManager
public import cargo.CargoManager
public import gem.GemManager
public import npm.NpmManager
public import pipx.PipxManager
public import pnpm.PnpmManager
public import rustup.RustupManager

// Re-exports for all platform managers (runtime detection handles availability)
public import brew.BrewManager
public import mas.MasManager
public import port.PortManager
public import softwareupdate.SoftwareUpdateManager
public import apt.AptManager
public import dnf.DnfManager
public import flatpak.FlatpakManager
public import pacman.PacmanManager
public import snap.SnapManager
public import choco.ChocoManager
public import scoop.ScoopManager
public import winget.WingetManager

/// Create a PackageManager from a Manager enum variant.
/// Returns null if no implementation exists for this Manager.
public fn createManager(manager: Manager): Box<dyn PackageManager>? {
  match manager {
    Manager.conda -> Box(CondaManager) as Box<dyn PackageManager>
    Manager.mise -> Box(MiseManager) as Box<dyn PackageManager>
    Manager.cargo -> Box(CargoManager) as Box<dyn PackageManager>
    Manager.gem -> Box(GemManager) as Box<dyn PackageManager>
    Manager.npm -> Box(NpmManager) as Box<dyn PackageManager>
    Manager.pipx -> Box(PipxManager) as Box<dyn PackageManager>
    Manager.pnpm -> Box(PnpmManager) as Box<dyn PackageManager>
    Manager.rustup -> Box(RustupManager) as Box<dyn PackageManager>
    Manager.brew -> Box(BrewManager) as Box<dyn PackageManager>
    Manager.mas -> Box(MasManager) as Box<dyn PackageManager>
    Manager.port -> Box(PortManager) as Box<dyn PackageManager>
    Manager.softwareupdate -> Box(SoftwareUpdateManager) as Box<dyn PackageManager>
    Manager.apt -> Box(AptManager) as Box<dyn PackageManager>
    Manager.dnf -> Box(DnfManager) as Box<dyn PackageManager>
    Manager.flatpak -> Box(FlatpakManager) as Box<dyn PackageManager>
    Manager.pacman -> Box(PacmanManager) as Box<dyn PackageManager>
    Manager.snap -> Box(SnapManager) as Box<dyn PackageManager>
    Manager.choco -> Box(ChocoManager) as Box<dyn PackageManager>
    Manager.scoop -> Box(ScoopManager) as Box<dyn PackageManager>
    Manager.winget -> Box(WingetManager) as Box<dyn PackageManager>
  }
}
