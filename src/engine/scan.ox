//! System scanning for installed package managers

import strum.IntoEnumIterator
import which.which

import super.managers.createManager
import super.types.{Action, Manager, ScanReport}

/// Scan the system for installed tools and detect their managers.
public fn scan(): ScanReport {
    let availableManagers = Manager.iter()
        .filter { which(it.as_ref().to_lowercase()).is_ok() }
        .collect()

    // Compute actionable managers (those with implementations AND actions)
    let actionableManagers = Manager.iter()
        .filter { which(it.as_ref().to_lowercase()).is_ok() }
        .filter_map { manager ->
            let pkgManager = createManager(manager)?
            let hasActions = !pkgManager.updateActions().is_empty()
                || !pkgManager.upgradeActions().is_empty()
            hasActions.then_some(manager)
        }
        .collect()

    ScanReport(
        availableManagers: availableManagers,
        actionableManagers: actionableManagers
    )
}

/// Get actions for managers detected in the scan.
/// Only returns actions for managers that were actually detected on the system.
public fn getActionsForScan(report: ScanReport): [Action] {
    report
        .availableManagers
        .iter()
        .filter_map { createManager(it) }
        .flat_map { pkgManager ->
            pkgManager
                .updateActions()
                .into_iter()
                .chain(pkgManager.upgradeActions())
        }
        .collect()
}

/// Get check actions for managers detected in the scan.
/// Returns actions to check for outdated packages without updating.
public fn getCheckActionsForScan(report: ScanReport): [Action] {
    report
        .availableManagers
        .iter()
        .filter_map { createManager(it) }
        .flat_map { it.checkActions() }
        .collect()
}

@[cfg(test)]
module tests {
    import super.*

    @[test]
    fn testScanActionableManagersSubsetOfAvailable() {
        let report = scan()

        // CRITICAL REGRESSION TEST: actionableManagers must be a subset of availableManagers
        for manager in report.actionableManagers {
            assert!(
                report.availableManagers.contains(manager),
                "Actionable manager {} must be in availableManagers",
                manager
            )
        }
    }

    @[test]
    fn testScanActionableManagersHaveImplementations() {
        let report = scan()

        // All actionable managers must have implementations (createManager returns Some)
        for manager in report.actionableManagers {
            let pkgManager = createManager(manager)
            assert!(
                pkgManager.is_some(),
                "Actionable manager {} must have an implementation",
                manager
            )
        }
    }

    @[test]
    fn testScanActionableManagersHaveActions() {
        let report = scan()

        // All actionable managers must have at least one action (update or upgrade)
        for manager in report.actionableManagers {
            let pkgManager =
                createManager(manager).expect("actionable manager should have implementation")
            let hasActions = !pkgManager.updateActions().is_empty()
                || !pkgManager.upgradeActions().is_empty()

            assert!(
                hasActions,
                "Actionable manager {} must have at least one action",
                manager
            )
        }
    }

    @[test]
    fn testScanAvailableManagersAreInPath() {
        let report = scan()

        // All available managers must have their binaries in PATH
        for manager in report.availableManagers {
            let binaryName = manager.as_ref().to_lowercase()
            let inPath = which(binaryName).is_ok()

            assert!(
                inPath,
                "Available manager {} binary '{}' must be in PATH",
                manager, binaryName
            )
        }
    }

    @[test]
    fn testGetActionsForScanReturnsActionsOnlyForAvailableManagers() {
        let report = scan()
        let actions = getActionsForScan(report)

        // All returned actions must be from managers that are in availableManagers
        for action in actions {
            assert!(
                report.availableManagers.contains(action.manager),
                "Action manager {} must be in availableManagers",
                action.manager
            )
        }
    }

    @[test]
    fn testScanConsistency() {
        let report = scan()

        // Run scan multiple times - results should be consistent (deterministic)
        let report2 = scan()

        assert_eq!(
            report.availableManagers, report2.availableManagers,
            "Available managers should be consistent across scans"
        )

        assert_eq!(
            report.actionableManagers, report2.actionableManagers,
            "Actionable managers should be consistent across scans"
        )
    }

    @[test]
    fn testScanActionableManagersNotEmptyWhenManagersAvailable() {
        let report = scan()

        // CRITICAL REGRESSION TEST: This would have caught the bug where
        // actionableManagers was set to Default::default() (empty set).
        // If we detect any package managers, at least some should be actionable.
        // Skip this assertion if no managers are available (e.g., in CI environments).
        if !report.availableManagers.is_empty() {
            assert!(
                !report.actionableManagers.is_empty(),
                "When package managers are available, at least some should be actionable. Found {} available managers but 0 actionable managers.",
                report.availableManagers.len()
            )
        }
    }
}
