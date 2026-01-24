//! System scanning for installed package managers

import strum.IntoEnumIterator
import which.which

import super.managers.create_manager
import super.types.{Action, Manager, ScanReport}

/// Scan the system for installed tools and detect their managers.
public fn scan(): ScanReport {
    let available_managers = Manager.iter()
        .filter({ manager -> which(manager.as_ref().to_lowercase()).is_ok() })
        .collect()

    // Compute actionable managers (those with implementations AND actions)
    let actionable_managers = Manager.iter()
        .filter({ manager -> which(manager.as_ref().to_lowercase()).is_ok() })
        .filter_map({ manager ->
            let pkg_manager = create_manager(&manager)?
            let has_actions = !pkg_manager.update_actions().is_empty()
                || !pkg_manager.upgrade_actions().is_empty()
            has_actions.then_some(manager)
        })
        .collect()

    ScanReport(
        available_managers: available_managers,
        actionable_managers: actionable_managers
    )
}

/// Get actions for managers detected in the scan.
/// Only returns actions for managers that were actually detected on the system.
public fn get_actions_for_scan(report: &ScanReport): Vec<Action> {
    report
        .available_managers
        .iter()
        .filter_map({ manager -> create_manager(manager) })
        .flat_map({ pkg_manager ->
            pkg_manager
                .update_actions()
                .into_iter()
                .chain(pkg_manager.upgrade_actions())
        })
        .collect()
}

/// Get check actions for managers detected in the scan.
/// Returns actions to check for outdated packages without updating.
public fn get_check_actions_for_scan(report: &ScanReport): Vec<Action> {
    report
        .available_managers
        .iter()
        .filter_map({ manager -> create_manager(manager) })
        .flat_map({ pkg_manager -> pkg_manager.check_actions() })
        .collect()
}

@[cfg(test)]
module tests {
    import super.*

    @[test]
    fn test_scan_actionable_managers_subset_of_available() {
        let report = scan()

        // CRITICAL REGRESSION TEST: actionable_managers must be a subset of available_managers
        for manager in &report.actionable_managers {
            assert!(
                report.available_managers.contains(manager),
                "Actionable manager {} must be in available_managers",
                manager
            )
        }
    }

    @[test]
    fn test_scan_actionable_managers_have_implementations() {
        let report = scan()

        // All actionable managers must have implementations (create_manager returns Some)
        for manager in &report.actionable_managers {
            let pkg_manager = create_manager(manager)
            assert!(
                pkg_manager.is_some(),
                "Actionable manager {} must have an implementation",
                manager
            )
        }
    }

    @[test]
    fn test_scan_actionable_managers_have_actions() {
        let report = scan()

        // All actionable managers must have at least one action (update or upgrade)
        for manager in &report.actionable_managers {
            let pkg_manager =
                create_manager(manager).expect("actionable manager should have implementation")
            let has_actions = !pkg_manager.update_actions().is_empty()
                || !pkg_manager.upgrade_actions().is_empty()

            assert!(
                has_actions,
                "Actionable manager {} must have at least one action",
                manager
            )
        }
    }

    @[test]
    fn test_scan_available_managers_are_in_path() {
        let report = scan()

        // All available managers must have their binaries in PATH
        for manager in &report.available_managers {
            let binary_name = manager.as_ref().to_lowercase()
            let in_path = which(&binary_name).is_ok()

            assert!(
                in_path,
                "Available manager {} binary '{}' must be in PATH",
                manager, binary_name
            )
        }
    }

    @[test]
    fn test_get_actions_for_scan_returns_actions_only_for_available_managers() {
        let report = scan()
        let actions = get_actions_for_scan(&report)

        // All returned actions must be from managers that are in available_managers
        for action in &actions {
            assert!(
                report.available_managers.contains(&action.manager),
                "Action manager {} must be in available_managers",
                action.manager
            )
        }
    }

    @[test]
    fn test_scan_consistency() {
        let report = scan()

        // Run scan multiple times - results should be consistent (deterministic)
        let report2 = scan()

        assert_eq!(
            report.available_managers, report2.available_managers,
            "Available managers should be consistent across scans"
        )

        assert_eq!(
            report.actionable_managers, report2.actionable_managers,
            "Actionable managers should be consistent across scans"
        )
    }

    @[test]
    fn test_scan_actionable_managers_not_empty_when_managers_available() {
        let report = scan()

        // CRITICAL REGRESSION TEST: This would have caught the bug where
        // actionable_managers was set to Default::default() (empty set).
        // If we detect any package managers, at least some should be actionable.
        // Skip this assertion if no managers are available (e.g., in CI environments).
        if !report.available_managers.is_empty() {
            assert!(
                !report.actionable_managers.is_empty(),
                "When package managers are available, at least some should be actionable. Found {} available managers but 0 actionable managers.",
                report.available_managers.len()
            )
        }
    }
}
