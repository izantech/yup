use std::fs;

use tracing::debug;
use which::which;

use super::detect::detect_manager;
use super::managers::create_manager;
use super::types::{Action, DetectedTool, Manager, ScanReport};

/// List of tools to scan for
const TOOLS: &[&str] = &[
    // System package managers
    "brew",
    "port",
    "mas",
    "softwareupdate",
    // Version managers
    "asdf",
    "mise",
    "pyenv",
    "rbenv",
    "rvm",
    "nvm",
    "fnm",
    "volta",
    "conda",
    // Node.js ecosystem
    "node",
    "npm",
    "pnpm",
    "yarn",
    "bun",
    // Python ecosystem
    "python3",
    "pip3",
    "pipx",
    "poetry",
    "uv",
    // Ruby ecosystem
    "ruby",
    "gem",
    // Rust ecosystem
    "rustup",
    "cargo",
    // Other languages
    "go",
    "composer",
];

/// Scan the system for installed tools and detect their managers
pub fn scan() -> ScanReport {
    let mut report = ScanReport::default();

    for &tool_name in TOOLS {
        if let Some(detected) = detect_tool(tool_name) {
            debug!(
                tool = tool_name,
                manager = ?detected.manager,
                path = %detected.path.display(),
                "Detected tool"
            );
            report.available_managers.insert(detected.manager);
            report.detected_tools.push(detected);
        }
    }

    // Compute actionable managers (those with implementations AND actions)
    for manager in &report.available_managers {
        if let Some(pkg_manager) = create_manager(*manager) {
            if !pkg_manager.update_actions().is_empty() || !pkg_manager.upgrade_actions().is_empty()
            {
                report.actionable_managers.insert(*manager);
            }
        }
    }

    report
}

fn detect_tool(name: &str) -> Option<DetectedTool> {
    let path = which(name).ok()?;

    // Resolve symlinks to get real path
    let resolved = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());

    // Package manager CLIs map directly to their manager regardless of installation path.
    // Only runtime binaries (node, python3, ruby) use path-based detection to determine
    // which version manager controls them.
    let manager = match name {
        // System package managers
        "brew" => Manager::Brew,
        "port" => Manager::Port,
        "mas" => Manager::Mas,
        "softwareupdate" => Manager::SoftwareUpdate,
        // Version managers
        "asdf" => Manager::Asdf,
        "mise" => Manager::Mise,
        "pyenv" => Manager::Pyenv,
        "rbenv" => Manager::Rbenv,
        "rvm" => Manager::Rvm,
        "nvm" => Manager::Nvm,
        "fnm" => Manager::Fnm,
        "volta" => Manager::Volta,
        "conda" => Manager::Conda,
        // Node.js package managers
        "npm" => Manager::Npm,
        "pnpm" => Manager::Pnpm,
        "yarn" => Manager::Yarn,
        "bun" => Manager::Bun,
        // Python package managers
        "pip3" => Manager::Pip,
        "pipx" => Manager::Pipx,
        "poetry" => Manager::Poetry,
        "uv" => Manager::Uv,
        // Ruby package managers
        "gem" => Manager::Gem,
        // Rust tools
        "rustup" => Manager::Rustup,
        "cargo" => Manager::Cargo,
        // Other
        "go" => Manager::Go,
        "composer" => Manager::Composer,
        // Runtime binaries - detect based on path to find their version manager
        // (e.g., node might be from nvm, fnm, volta, asdf, brew, etc.)
        _ => detect_manager(&resolved),
    };

    Some(DetectedTool {
        name: name.to_string(),
        path: resolved,
        manager,
    })
}

/// Get actions for managers detected in the scan.
/// Only returns actions for managers that were actually detected on the system.
pub fn get_actions_for_scan(report: &ScanReport) -> Vec<Action> {
    let mut actions = Vec::new();

    for manager in &report.available_managers {
        if let Some(pkg_manager) = create_manager(*manager) {
            actions.extend(pkg_manager.update_actions());
            actions.extend(pkg_manager.upgrade_actions());
        }
    }

    actions
}

/// Get check actions for managers detected in the scan.
/// Returns actions to check for outdated packages without updating.
pub fn get_check_actions_for_scan(report: &ScanReport) -> Vec<Action> {
    let mut actions = Vec::new();

    for manager in &report.available_managers {
        if let Some(pkg_manager) = create_manager(*manager) {
            actions.extend(pkg_manager.check_actions());
        }
    }

    actions
}
