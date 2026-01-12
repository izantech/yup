use strum::IntoEnumIterator;
use which::which;

use super::managers::create_manager;
use super::types::{Action, Manager, ScanReport};

/// Scan the system for installed tools and detect their managers.
pub fn scan() -> ScanReport {
    let available_managers = Manager::iter()
        .filter(|manager| which(&manager.as_ref().to_lowercase()).is_ok())
        .collect();

    ScanReport {
        available_managers,
        actionable_managers: Default::default(),
    }
}

/// Get actions for managers detected in the scan.
/// Only returns actions for managers that were actually detected on the system.
pub fn get_actions_for_scan(report: &ScanReport) -> Vec<Action> {
    report
        .available_managers
        .iter()
        .filter_map(|&manager| create_manager(manager))
        .flat_map(|pkg_manager| {
            pkg_manager
                .update_actions()
                .into_iter()
                .chain(pkg_manager.upgrade_actions())
        })
        .collect()
}

/// Get check actions for managers detected in the scan.
/// Returns actions to check for outdated packages without updating.
pub fn get_check_actions_for_scan(report: &ScanReport) -> Vec<Action> {
    report
        .available_managers
        .iter()
        .filter_map(|&manager| create_manager(manager))
        .flat_map(|pkg_manager| pkg_manager.check_actions())
        .collect()
}
