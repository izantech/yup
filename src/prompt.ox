//! Interactive configuration wizard

import std.collections.HashSet
import dialoguer.{ Confirm, MultiSelect, theme.ColorfulTheme }
import anyhow.Result
import crate.config.Config
import crate.engine.{ Action, Manager, ScanReport, getActionsForScan }

/// Run the configuration wizard
/// Returns (Config, shouldExecute: bool)
public fn runWizard(report: ScanReport): Result<(Config, bool)> {
  println!("\n=== yup - Development Tool Updater ===\n")

  // Get actionable managers sorted (only those with actual update/upgrade actions)
  var managers: [Manager] = report.actionableManagers.iter().copied().collect()
  managers.sortByKey {
    "$it"
  }

  if managers.isEmpty() {
    println!("No package managers with available actions detected on this system.")
    return Ok((Config.default(), false))
  }

  // Show detected tools
  println!("Detected {} package managers:\n", managers.len())

  // Multi-select for managers
  let managerNames: [String] = managers.iter().map {
    "$it"
  }.collect()
  let defaults: [bool] = managers.iter().map {
    true
  }.collect()

  let selections = MultiSelect.with_theme(ColorfulTheme.default()).with_prompt("Select managers to update (Space: toggle, a: toggle all, Enter: confirm)").items(managerNames).defaults(defaults).interact()?

  if selections.isEmpty() {
    println!("No managers selected. Exiting.")
    return Ok((Config.default(), false))
  }

  let enabledManagers: [Manager] = selections.iter().map {
    managers[*it]
  }.collect()

  // Build config
  let config = Config(enabledManagers: enabledManagers.clone())

  // Preview commands
  println!("\n--- Commands to run ---\n")
  let actions = getFilteredActions(config, report)

  if actions.isEmpty() {
    println!("  (no actions available for selected managers)\n")
  } else {
    for action in actions {
      let sudoMarker = if action.requiresPrivilege { " [sudo]" } else { "" }
      println!("  {} - {}{}", action.command, action.description, sudoMarker)
    }
    println!()
  }

  // Confirm execution
  let shouldExecute = if actions.isEmpty() {
    Confirm.with_theme(ColorfulTheme.default()).with_prompt("Save this configuration?").default(true).interact()?
  } else {
    Confirm.with_theme(ColorfulTheme.default()).with_prompt("Save configuration and run these commands?").default(true).interact()?
  }

  Ok((config, shouldExecute))
}

/// Get actions filtered by config
public fn getFilteredActions(config: Config, report: ScanReport): [Action] {
  let allActions = getActionsForScan(report)
  let enabledSet: HashSet<Manager> = config.enabledManagerSet()

  allActions.intoIter().filter {
    enabledSet.contains(it.manager)
  }.collect()
}
