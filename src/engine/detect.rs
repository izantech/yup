use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use directories::BaseDirs;

use super::types::Manager;

/// Cached Homebrew prefix (computed once via `brew --prefix`)
static BREW_PREFIX: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Get the Homebrew prefix, caching the result.
/// Returns None if brew is not installed or command fails.
fn get_brew_prefix() -> Option<&'static PathBuf> {
    BREW_PREFIX
        .get_or_init(|| {
            Command::new("brew")
                .arg("--prefix")
                .output()
                .ok()
                .filter(|o| o.status.success())
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| PathBuf::from(s.trim()))
        })
        .as_ref()
}

/// Detect which manager a tool belongs to based on its resolved path.
///
/// Priority:
/// 1. Environment variables (user-configured paths)
/// 2. Path patterns (default locations)
pub fn detect_manager(path: &Path) -> Manager {
    // Get base directories for home path matching
    let base_dirs = BaseDirs::new();
    let home = base_dirs.as_ref().map(|b| b.home_dir());

    // === Environment variable checks (highest priority) ===

    // NVM_DIR for nvm
    if let Some(nvm_dir) = std::env::var_os("NVM_DIR") {
        let nvm_path = PathBuf::from(nvm_dir);
        if path.starts_with(&nvm_path) {
            return Manager::Nvm;
        }
    }

    // VOLTA_HOME for volta
    if let Some(volta_home) = std::env::var_os("VOLTA_HOME") {
        let volta_path = PathBuf::from(volta_home);
        if path.starts_with(&volta_path) {
            return Manager::Volta;
        }
    }

    // ASDF_DATA_DIR for asdf (or default ~/.asdf)
    if let Some(asdf_dir) = std::env::var_os("ASDF_DATA_DIR") {
        let asdf_path = PathBuf::from(asdf_dir);
        if path.starts_with(&asdf_path) {
            return Manager::Asdf;
        }
    }

    // FNM_DIR or FNM_MULTISHELL_PATH for fnm
    if let Some(fnm_dir) = std::env::var_os("FNM_DIR") {
        let fnm_path = PathBuf::from(fnm_dir);
        if path.starts_with(&fnm_path) {
            return Manager::Fnm;
        }
    }
    if let Some(fnm_multishell) = std::env::var_os("FNM_MULTISHELL_PATH") {
        let fnm_path = PathBuf::from(fnm_multishell);
        if path.starts_with(&fnm_path) {
            return Manager::Fnm;
        }
    }

    // PYENV_ROOT for pyenv (or default ~/.pyenv)
    if let Some(pyenv_root) = std::env::var_os("PYENV_ROOT") {
        let pyenv_path = PathBuf::from(pyenv_root);
        if path.starts_with(&pyenv_path) {
            return Manager::Pyenv;
        }
    }

    // RBENV_ROOT for rbenv (or default ~/.rbenv)
    if let Some(rbenv_root) = std::env::var_os("RBENV_ROOT") {
        let rbenv_path = PathBuf::from(rbenv_root);
        if path.starts_with(&rbenv_path) {
            return Manager::Rbenv;
        }
    }

    // CONDA_PREFIX for conda/mamba
    if let Some(conda_prefix) = std::env::var_os("CONDA_PREFIX") {
        let conda_path = PathBuf::from(conda_prefix);
        if path.starts_with(&conda_path) {
            return Manager::Conda;
        }
    }

    // CHOCOLATEYINSTALL for Chocolatey (Windows)
    if let Some(choco_install) = std::env::var_os("CHOCOLATEYINSTALL") {
        let choco_path = PathBuf::from(choco_install);
        if path.starts_with(&choco_path) {
            return Manager::Choco;
        }
    }

    // SCOOP for Scoop user installation (Windows)
    if let Some(scoop_dir) = std::env::var_os("SCOOP") {
        let scoop_path = PathBuf::from(scoop_dir);
        if path.starts_with(&scoop_path) {
            return Manager::Scoop;
        }
    }

    // SCOOP_GLOBAL for Scoop global installation (Windows)
    if let Some(scoop_global) = std::env::var_os("SCOOP_GLOBAL") {
        let scoop_path = PathBuf::from(scoop_global);
        if path.starts_with(&scoop_path) {
            return Manager::Scoop;
        }
    }

    // === Path pattern checks (fallback to default locations) ===

    if let Some(home_dir) = home {
        // Version managers - default locations
        let nvm_default = home_dir.join(".nvm");
        if path.starts_with(&nvm_default) {
            return Manager::Nvm;
        }

        let fnm_default = home_dir.join(".fnm");
        let fnm_xdg = home_dir.join(".local/share/fnm");
        if path.starts_with(&fnm_default) || path.starts_with(&fnm_xdg) {
            return Manager::Fnm;
        }

        let volta_default = home_dir.join(".volta");
        if path.starts_with(&volta_default) {
            return Manager::Volta;
        }

        let asdf_default = home_dir.join(".asdf");
        if path.starts_with(&asdf_default) {
            return Manager::Asdf;
        }

        let pyenv_default = home_dir.join(".pyenv");
        if path.starts_with(&pyenv_default) {
            return Manager::Pyenv;
        }

        let rbenv_default = home_dir.join(".rbenv");
        if path.starts_with(&rbenv_default) {
            return Manager::Rbenv;
        }

        let rvm_default = home_dir.join(".rvm");
        if path.starts_with(&rvm_default) {
            return Manager::Rvm;
        }

        // Conda/Mamba variants
        let miniconda = home_dir.join("miniconda3");
        let anaconda = home_dir.join("anaconda3");
        let mambaforge = home_dir.join("mambaforge");
        let miniforge = home_dir.join("miniforge");
        let miniforge3 = home_dir.join("miniforge3");
        let micromamba = home_dir.join("micromamba");
        if path.starts_with(&miniconda)
            || path.starts_with(&anaconda)
            || path.starts_with(&mambaforge)
            || path.starts_with(&miniforge)
            || path.starts_with(&miniforge3)
            || path.starts_with(&micromamba)
        {
            return Manager::Conda;
        }

        // Cargo/Rustup - ~/.cargo/bin
        let cargo_bin = home_dir.join(".cargo/bin");
        if path.starts_with(&cargo_bin) {
            return Manager::Cargo;
        }

        // Scoop user default path (under home directory)
        #[cfg(target_os = "windows")]
        {
            let scoop_user = home_dir.join("scoop");
            if path.starts_with(&scoop_user) {
                return Manager::Scoop;
            }
        }
    }

    // === Windows-specific path patterns ===
    #[cfg(target_os = "windows")]
    {
        // nvm-windows uses %APPDATA%\nvm
        if let Some(appdata) = std::env::var_os("APPDATA") {
            let appdata_path = PathBuf::from(&appdata);

            let nvm_windows = appdata_path.join("nvm");
            if path.starts_with(&nvm_windows) {
                return Manager::Nvm;
            }

            // fnm on Windows may use %APPDATA%\fnm
            let fnm_appdata = appdata_path.join("fnm");
            if path.starts_with(&fnm_appdata) {
                return Manager::Fnm;
            }
        }

        // fnm may also use %LOCALAPPDATA%\fnm
        if let Some(localappdata) = std::env::var_os("LOCALAPPDATA") {
            let fnm_localappdata = PathBuf::from(localappdata).join("fnm");
            if path.starts_with(&fnm_localappdata) {
                return Manager::Fnm;
            }
        }

        // Chocolatey default path
        let choco_default = Path::new(r"C:\ProgramData\chocolatey");
        if path.starts_with(choco_default) {
            return Manager::Choco;
        }

        // Scoop global default path
        let scoop_global_default = Path::new(r"C:\ProgramData\scoop");
        if path.starts_with(scoop_global_default) {
            return Manager::Scoop;
        }

        // Windows system paths
        let windows_system32 = Path::new(r"C:\Windows\System32");
        let windows_dir = Path::new(r"C:\Windows");
        if path.starts_with(windows_system32) || path.starts_with(windows_dir) {
            return Manager::System;
        }
    }

    // === Unix-specific path patterns ===
    #[cfg(not(target_os = "windows"))]
    {
        // Homebrew paths - dynamic prefix first, then hardcoded fallbacks
        if let Some(brew_prefix) = get_brew_prefix() {
            if path.starts_with(brew_prefix) {
                return Manager::Brew;
            }
            // Also check Cellar subdirectory under dynamic prefix
            let cellar = brew_prefix.join("Cellar");
            if path.starts_with(&cellar) {
                return Manager::Brew;
            }
        }
        // Hardcoded fallbacks for when brew command is not available
        let opt_homebrew = Path::new("/opt/homebrew");
        let usr_local_cellar = Path::new("/usr/local/Cellar");
        let usr_local_opt = Path::new("/usr/local/opt");
        let usr_local = Path::new("/usr/local");
        if path.starts_with(opt_homebrew)
            || path.starts_with(usr_local_cellar)
            || path.starts_with(usr_local_opt)
            || path.starts_with(usr_local)
        {
            return Manager::Brew;
        }

        // MacPorts
        let opt_local = Path::new("/opt/local");
        if path.starts_with(opt_local) {
            return Manager::Port;
        }

        // System paths
        let usr_bin = Path::new("/usr/bin");
        let usr_sbin = Path::new("/usr/sbin");
        let bin = Path::new("/bin");
        if path.starts_with(usr_bin) || path.starts_with(usr_sbin) || path.starts_with(bin) {
            return Manager::System;
        }
    }

    Manager::Unknown
}
