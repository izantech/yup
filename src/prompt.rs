use std::collections::HashSet;

use dialoguer::{Confirm, MultiSelect, theme::ColorfulTheme};

use crate::config::Config;
use crate::engine::{Manager, ScanReport, get_actions_for_scan};

/// Run the configuration wizard
/// Returns (Config, should_execute: bool)
pub fn run_wizard(report: &ScanReport) -> anyhow::Result<(Config, bool)> {
    println!("\n=== yup - Development Tool Updater ===\n");

    // Get actionable managers sorted (only those with actual update/upgrade actions)
    let mut managers: Vec<Manager> = report.actionable_managers.iter().copied().collect();
    managers.sort_by_key(|m| m.to_string());

    if managers.is_empty() {
        println!("No package managers with available actions detected on this system.");
        return Ok((Config::default(), false));
    }

    // Show detected tools
    println!("Detected {} package managers:\n", managers.len());

    // Multi-select for managers
    let manager_names: Vec<String> = managers.iter().map(|m| m.to_string()).collect();
    let defaults: Vec<bool> = vec![true; managers.len()];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select managers to update (Space: toggle, a: toggle all, Enter: confirm)")
        .items(&manager_names)
        .defaults(&defaults)
        .interact()?;

    if selections.is_empty() {
        println!("No managers selected. Exiting.");
        return Ok((Config::default(), false));
    }

    let enabled_managers: Vec<Manager> = selections.iter().map(|&i| managers[i]).collect();

    // Build config
    let config = Config {
        enabled_managers: enabled_managers.clone(),
    };

    // Preview commands
    println!("\n--- Commands to run ---\n");
    let actions = get_filtered_actions(&config, report);

    if actions.is_empty() {
        println!("  (no actions available for selected managers)\n");
    } else {
        for action in &actions {
            let sudo_marker = if action.requires_privilege {
                " [sudo]"
            } else {
                ""
            };
            println!(
                "  {} - {}{}",
                action.command, action.description, sudo_marker
            );
        }
        println!();
    }

    // Confirm execution
    let should_execute = if actions.is_empty() {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Save this configuration?")
            .default(true)
            .interact()?
    } else {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Save configuration and run these commands?")
            .default(true)
            .interact()?
    };

    Ok((config, should_execute))
}

/// Get actions filtered by config
pub fn get_filtered_actions(
    config: &Config,
    report: &ScanReport,
) -> Vec<crate::engine::types::Action> {
    let all_actions = get_actions_for_scan(report);
    let enabled_set: HashSet<Manager> = config.enabled_manager_set();

    all_actions
        .into_iter()
        .filter(|a| enabled_set.contains(&a.manager))
        .collect()
}
