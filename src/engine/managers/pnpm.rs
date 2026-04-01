//! pnpm package manager

use super::{Action, Manager, PackageManager};

/// pnpm package manager
pub struct PnpmManager;

/// Check if corepack is available
fn is_corepack_available() -> bool {
    which::which("corepack").is_ok()
}

impl PackageManager for PnpmManager {
    fn update_actions(&self) -> Vec<Action> {
        if is_corepack_available() {
            let cmd = if cfg!(target_os = "windows") {
                "set COREPACK_ENABLE_DOWNLOAD_PROMPT=0 && corepack install -g pnpm@latest"
            } else {
                "COREPACK_ENABLE_DOWNLOAD_PROMPT=0 corepack install -g pnpm@latest"
            };
            vec![Action::new(
                Manager::Pnpm,
                cmd,
                "Update pnpm via corepack",
                false,
            )]
        } else {
            vec![Action::new(
                Manager::Pnpm,
                "pnpm self-update",
                "Update pnpm itself",
                false,
            )]
        }
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Pnpm,
            "pnpm update -g",
            "Update global pnpm packages",
            false,
        )]
    }

    fn check_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::Pnpm,
            "pnpm outdated -g",
            "Check for outdated global pnpm packages",
            false,
        )]
    }
}
