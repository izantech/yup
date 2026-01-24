//! Package manager implementations

import super.types.{Action, Manager}

/// Trait defining the interface for package manager implementations.
public trait PackageManager {
    fn update_actions(): Vec<Action>
    fn upgrade_actions(): Vec<Action>
    fn check_actions(): Vec<Action>
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
/// Returns None if no implementation exists for this Manager.
public fn create_manager(manager: Manager): Box<dyn PackageManager>? {
    match manager {
        Manager.Conda -> Some(Box.new(CondaManager) as Box<dyn PackageManager>)
        Manager.Mise -> Some(Box.new(MiseManager) as Box<dyn PackageManager>)
        Manager.Cargo -> Some(Box.new(CargoManager) as Box<dyn PackageManager>)
        Manager.Gem -> Some(Box.new(GemManager) as Box<dyn PackageManager>)
        Manager.Npm -> Some(Box.new(NpmManager) as Box<dyn PackageManager>)
        Manager.Pipx -> Some(Box.new(PipxManager) as Box<dyn PackageManager>)
        Manager.Pnpm -> Some(Box.new(PnpmManager) as Box<dyn PackageManager>)
        Manager.Rustup -> Some(Box.new(RustupManager) as Box<dyn PackageManager>)
        Manager.Brew -> Some(Box.new(BrewManager) as Box<dyn PackageManager>)
        Manager.Mas -> Some(Box.new(MasManager) as Box<dyn PackageManager>)
        Manager.Port -> Some(Box.new(PortManager) as Box<dyn PackageManager>)
        Manager.SoftwareUpdate -> Some(Box.new(SoftwareUpdateManager) as Box<dyn PackageManager>)
        Manager.Apt -> Some(Box.new(AptManager) as Box<dyn PackageManager>)
        Manager.Dnf -> Some(Box.new(DnfManager) as Box<dyn PackageManager>)
        Manager.Flatpak -> Some(Box.new(FlatpakManager) as Box<dyn PackageManager>)
        Manager.Pacman -> Some(Box.new(PacmanManager) as Box<dyn PackageManager>)
        Manager.Snap -> Some(Box.new(SnapManager) as Box<dyn PackageManager>)
        Manager.Choco -> Some(Box.new(ChocoManager) as Box<dyn PackageManager>)
        Manager.Scoop -> Some(Box.new(ScoopManager) as Box<dyn PackageManager>)
        Manager.Winget -> Some(Box.new(WingetManager) as Box<dyn PackageManager>)
    }
}
