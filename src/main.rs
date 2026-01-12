mod cli;
mod config;
mod engine;
mod prompt;
#[cfg(unix)]
mod sudo;

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Stdio;
use std::time::{Duration, Instant};

use clap::Parser;
use cli::{Cli, Command};
use config::Config;
use directories::ProjectDirs;
use engine::{filter_actions, get_actions_for_scan, get_check_actions_for_scan, scan};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as ProcessCommand;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Set up file logging with daily rotation.
/// Returns the log directory path on success.
fn setup_file_logging() -> Option<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "", "yup")?;
    let log_dir = proj_dirs.data_local_dir();

    // Create log directory if it doesn't exist
    std::fs::create_dir_all(log_dir).ok()?;

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("yup")
        .filename_suffix("log")
        .build(log_dir)
        .ok()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false),
        )
        .init();

    Some(log_dir.to_path_buf())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Set up file logging (silent failure is OK - logging is optional)
    let _log_dir = setup_file_logging();

    let args = Cli::parse();

    match args.command {
        Some(Command::Config) => {
            // Force re-run wizard
            run_wizard_flow(&args).await?;
        }
        Some(Command::Log) => {
            show_last_log()?;
        }
        None => {
            // Check for --status flag first
            if args.status {
                run_status(&args).await?;
            } else if Config::exists() {
                // Subsequent run: use saved config
                run_with_config(&args).await?;
            } else if args.yes {
                // No config but -y flag: run with all detected managers
                run_without_config(&args).await?;
            } else {
                // First run: show wizard
                run_wizard_flow(&args).await?;
            }
        }
    }

    Ok(())
}

async fn run_wizard_flow(args: &Cli) -> anyhow::Result<()> {
    let report = scan::scan();
    let (config, should_execute) = prompt::run_wizard(&report)?;

    if should_execute {
        config.save()?;
        if let Some(path) = Config::path() {
            println!("\nConfiguration saved to {}\n", path.display());
        }

        let actions = prompt::get_filtered_actions(&config, &report);
        if !actions.is_empty() {
            run_actions(&actions, args).await?;
        }
    } else {
        println!("Configuration cancelled.");
    }

    Ok(())
}

/// Run with all detected managers (no config, -y flag)
async fn run_without_config(args: &Cli) -> anyhow::Result<()> {
    // Scan and use detected managers
    let report = scan::scan();
    let all_actions = get_actions_for_scan(&report);

    // Apply CLI filters
    let actions = filter_actions(all_actions, args.only.as_deref(), args.skip.as_deref());

    if actions.is_empty() {
        println!("No actions to run.");
        return Ok(());
    }

    run_actions(&actions, args).await
}

/// Run status check - show outdated packages without updating
async fn run_status(args: &Cli) -> anyhow::Result<()> {
    let report = scan::scan();
    let all_actions = get_check_actions_for_scan(&report);

    // Apply CLI filters
    let actions = filter_actions(all_actions, args.only.as_deref(), args.skip.as_deref());

    if actions.is_empty() {
        println!("No status checks available for detected managers.");
        return Ok(());
    }

    // Check for privileged actions (Unix only)
    #[cfg(unix)]
    let needs_sudo = actions.iter().any(|a| a.requires_privilege);
    #[cfg(not(unix))]
    let needs_sudo = false;

    // Refresh sudo credentials if needed (Unix only)
    ensure_sudo_credentials(needs_sudo).await?;

    println!("Checking for outdated packages...\n");

    for action in &actions {
        println!("[{}] {}", action.manager, action.description);

        // Prepend sudo if needed on Unix
        #[cfg(unix)]
        let cmd = if action.requires_privilege {
            sudo::with_sudo(&action.command)
        } else {
            action.command.clone()
        };
        #[cfg(not(unix))]
        let cmd = action.command.clone();

        match execute_command(&cmd, OutputMode::Stream).await {
            Ok(_) => {}
            Err(e) => {
                println!("  Error: {}\n", e);
            }
        }
        println!();
    }

    Ok(())
}

async fn run_with_config(args: &Cli) -> anyhow::Result<()> {
    let config = Config::load().unwrap_or_default();
    let report = scan::scan();

    // Build actions from config
    let actions = prompt::get_filtered_actions(&config, &report);

    // Apply CLI overrides using filter_actions
    let actions = filter_actions(actions, args.only.as_deref(), args.skip.as_deref());

    if actions.is_empty() {
        println!("No actions to run with current configuration.");
        println!("Use 'yup config' to reconfigure.");
        return Ok(());
    }

    // Show summary
    println!("Running {} action(s)...\n", actions.len());

    run_actions(&actions, args).await
}

async fn run_actions(actions: &[engine::types::Action], args: &Cli) -> anyhow::Result<()> {
    // Display planned actions
    println!("Planned actions ({}):", actions.len());
    for (i, action) in actions.iter().enumerate() {
        let sudo_marker = if action.requires_privilege {
            " [sudo]"
        } else {
            ""
        };
        println!(
            "  {}. {}: {} - {}{}",
            i + 1,
            action.manager,
            action.command,
            action.description,
            sudo_marker
        );
    }
    println!();

    // Check for privileged actions (Unix only)
    #[cfg(unix)]
    let needs_sudo = actions.iter().any(|a| a.requires_privilege);
    #[cfg(not(unix))]
    let needs_sudo = false;

    // Show sudo warning if applicable
    #[cfg(unix)]
    if needs_sudo {
        let privileged_count = actions.iter().filter(|a| a.requires_privilege).count();
        println!(
            "Note: {} action(s) require sudo privileges.",
            privileged_count
        );
        println!();
    }

    // If dry-run, just show what would happen
    if args.dry_run {
        println!("[DRY RUN] Would execute the above actions.");
        return Ok(());
    }

    // Prompt for confirmation unless --yes is set
    if !args.yes {
        print!("Proceed with execution? [Y/n] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().eq_ignore_ascii_case("n") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!();

    // Refresh sudo credentials if needed (Unix only)
    ensure_sudo_credentials(needs_sudo).await?;

    let total = actions.len();
    let mut success_count = 0;
    let mut failed_actions: Vec<(usize, String, String)> = Vec::new();

    // Create progress bar
    let progress = ProgressBar::new(total as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:25.cyan/dim}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=> "),
    );
    progress.enable_steady_tick(Duration::from_millis(100));

    for (i, action) in actions.iter().enumerate() {
        let start = Instant::now();

        // Determine the actual command to run (prepend sudo if needed on Unix)
        #[cfg(unix)]
        let cmd_to_run = if action.requires_privilege {
            sudo::with_sudo(&action.command)
        } else {
            action.command.clone()
        };
        #[cfg(not(unix))]
        let cmd_to_run = action.command.clone();

        // Update progress bar message with current action
        progress.set_message(format!("{} - {}", action.command, action.description));

        // Execute command with progress integration
        let result = execute_command(
            &cmd_to_run,
            OutputMode::Progress {
                progress: &progress,
                verbose: args.verbose,
            },
        )
        .await;

        let elapsed = start.elapsed();

        match result {
            Ok(true) => {
                success_count += 1;
                if args.verbose {
                    progress.suspend(|| {
                        println!("      -> Done ({:.1}s)", elapsed.as_secs_f64());
                    });
                }
            }
            Ok(false) => {
                failed_actions.push((i + 1, action.command.clone(), "Command failed".to_string()));
                if args.verbose {
                    progress.suspend(|| {
                        println!("      -> FAILED ({:.1}s)", elapsed.as_secs_f64());
                    });
                }
            }
            Err(e) => {
                failed_actions.push((i + 1, action.command.clone(), e.to_string()));
                if args.verbose {
                    progress.suspend(|| {
                        println!("      -> ERROR: {}", e);
                    });
                }
            }
        }

        progress.inc(1);
    }

    // Finish progress bar
    if success_count == total {
        progress.finish_with_message("All actions completed successfully");
    } else {
        progress.finish_with_message(format!(
            "{}/{} succeeded, {} failed",
            success_count,
            total,
            total - success_count
        ));
    }

    println!();
    println!("Completed: {}/{} actions succeeded", success_count, total);

    // Show failed actions summary
    if !failed_actions.is_empty() {
        println!("\nFailed actions:");
        for (idx, cmd, err) in &failed_actions {
            println!("  {}. {} - {}", idx, cmd, err);
        }
    }

    Ok(())
}

/// Create a shell command for the current platform.
/// On Windows uses cmd.exe /C, on Unix uses sh -c.
fn create_shell_command(cmd_str: &str) -> ProcessCommand {
    #[cfg(windows)]
    {
        let mut cmd = ProcessCommand::new("cmd");
        cmd.args(["/C", cmd_str]);
        cmd
    }
    #[cfg(not(windows))]
    {
        let mut cmd = ProcessCommand::new("sh");
        cmd.args(["-c", cmd_str]);
        cmd
    }
}

enum OutputMode<'a> {
    Stream,
    Progress {
        progress: &'a ProgressBar,
        verbose: bool,
    },
}

/// Execute a shell command with optional progress bar integration.
/// - Stream: always prints stdout/stderr as they arrive.
/// - Progress: in verbose mode streams output above the progress bar;
///   otherwise captures stderr and only shows it on failure.
async fn execute_command(cmd_str: &str, mode: OutputMode<'_>) -> anyhow::Result<bool> {
    let mut cmd = create_shell_command(cmd_str);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let (progress, verbose, stream) = match mode {
        OutputMode::Stream => (None, false, true),
        OutputMode::Progress { progress, verbose } => (Some(progress), verbose, false),
    };

    let stdout_future = async {
        if let Some(stdout) = child.stdout.take() {
            let mut reader = BufReader::new(stdout).lines();
            while let Some(line) = reader.next_line().await? {
                if stream {
                    println!("      {}", line);
                } else if let Some(progress) = progress {
                    if verbose {
                        progress.suspend(|| println!("      {}", line));
                    }
                }
            }
        }
        Ok::<(), anyhow::Error>(())
    };

    let stderr_future = async {
        let mut stderr_output = Vec::new();
        if let Some(stderr) = child.stderr.take() {
            let mut reader = BufReader::new(stderr).lines();
            while let Some(line) = reader.next_line().await? {
                if stream {
                    println!("      [err] {}", line);
                } else if let Some(progress) = progress {
                    stderr_output.push(line.clone());
                    if verbose {
                        progress.suspend(|| println!("      [err] {}", line));
                    }
                }
            }
        }
        Ok::<Vec<String>, anyhow::Error>(stderr_output)
    };

    let (stdout_result, stderr_result) = tokio::join!(stdout_future, stderr_future);
    stdout_result?;
    let stderr_output = stderr_result?;

    let status = child.wait().await?;

    // If command failed and not verbose, print stderr for debugging
    if let OutputMode::Progress { progress, verbose } = mode {
        if !status.success() && !verbose && !stderr_output.is_empty() {
            progress.suspend(|| {
                for line in &stderr_output {
                    println!("      [err] {}", line);
                }
            });
        }
    }

    Ok(status.success())
}

#[cfg(unix)]
async fn ensure_sudo_credentials(needs_sudo: bool) -> anyhow::Result<()> {
    if !needs_sudo {
        return Ok(());
    }

    if !sudo::is_sudo_available() {
        anyhow::bail!("Sudo is required but not available on this system");
    }

    if !sudo::has_valid_credentials().await {
        println!("Refreshing sudo credentials...");
        if !sudo::refresh_credentials().await? {
            anyhow::bail!("Failed to obtain sudo credentials");
        }
        println!();
    }

    Ok(())
}

#[cfg(not(unix))]
async fn ensure_sudo_credentials(_needs_sudo: bool) -> anyhow::Result<()> {
    Ok(())
}

fn show_last_log() -> anyhow::Result<()> {
    let proj_dirs = ProjectDirs::from("", "", "yup")
        .ok_or_else(|| anyhow::anyhow!("Cannot determine log directory"))?;
    let log_dir = proj_dirs.data_local_dir();

    // Find the most recent log file
    let mut log_files: Vec<_> = std::fs::read_dir(log_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "log")
                .unwrap_or(false)
        })
        .collect();

    if log_files.is_empty() {
        println!("No log files found in {}", log_dir.display());
        return Ok(());
    }

    // Sort by modification time (most recent first)
    log_files.sort_by(|a, b| {
        let a_time = a.metadata().and_then(|m| m.modified()).ok();
        let b_time = b.metadata().and_then(|m| m.modified()).ok();
        b_time.cmp(&a_time)
    });

    let latest_log = &log_files[0];
    println!("=== {} ===\n", latest_log.path().display());

    let content = std::fs::read_to_string(latest_log.path())?;
    println!("{}", content);

    Ok(())
}
