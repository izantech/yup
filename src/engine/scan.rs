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
    // Version managers (with global upgrade support)
    "mise",
    "conda",
    // Node.js ecosystem
    "npm",
    "pnpm",
    // Python ecosystem (pipx only - pip intentionally excluded)
    "pipx",
    // Ruby ecosystem
    "gem",
    // Rust ecosystem
    "rustup",
    "cargo",
    // Windows package managers
    "choco",
    "winget",
    "scoop",
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
    let manager = match name {
        // System package managers
        "brew" => Manager::Brew,
        "port" => Manager::Port,
        "mas" => Manager::Mas,
        "softwareupdate" => Manager::SoftwareUpdate,
        // Version managers (with global upgrade support)
        "mise" => Manager::Mise,
        "conda" => Manager::Conda,
        // Node.js package managers
        "npm" => Manager::Npm,
        "pnpm" => Manager::Pnpm,
        // Python package managers
        "pipx" => Manager::Pipx,
        // Ruby package managers
        "gem" => Manager::Gem,
        // Rust tools
        "rustup" => Manager::Rustup,
        "cargo" => Manager::Cargo,
        // Windows package managers
        "choco" => Manager::Choco,
        "winget" => Manager::Winget,
        "scoop" => Manager::Scoop,
        // Unknown tools - try path-based detection
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
