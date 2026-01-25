external module cli
external module config
external module engine
external module prompt
external module sudo

import anyhow.{ Result, Error, anyhow }
import clap.Parser
import directories.ProjectDirs
import indicatif.{ ProgressBar, ProgressStyle }
import std.env
import std.io
import std.io.Write
import std.path.PathBuf
import std.process.Stdio
import std.time.{ Duration, Instant }
import tokio.io.{ AsyncBufReadExt, BufReader }
import tokio.process.Command as ProcessCommand
import tokio.runtime.Runtime
import tracing_appender.rolling.{ RollingFileAppender, Rotation }
import tracing_subscriber.{ layer.SubscriberExt, util.SubscriberInitExt }
import crate.cli.{ Cli, Command }
import crate.config.Config
import crate.engine.{ Action, filterActions, getActionsForScan, getCheckActionsForScan, scan }
import crate.prompt.{ getFilteredActions, runWizard }
import crate.sudo

fn isWindows(): Bool { env.consts.OS == "windows" }

/// Set up file logging with daily rotation.
/// Returns the log directory path on success.
fn setupFileLogging(): PathBuf? {
  let projDirs = ProjectDirs.from("", "", "yup")?
  let logDir = projDirs.dataLocalDir()

  // Create log directory if it doesn't exist
  std.fs.createDirAll(logDir).ok()?

  let fileAppender = RollingFileAppender.builder()
    .rotation(Rotation.DAILY)
    .filenamePrefix("yup")
    .filenameSuffix("log")
    .build(logDir)
    .ok()?

  tracing_subscriber
    .registry()
    .with(tracing_subscriber.fmt.layer().withWriter(fileAppender).withAnsi(false))
    .init()

  Some(logDir.toPathBuf())
}

fn main(): Result<()> {
  let runtime = Runtime.new().unwrap()
  runtime.blockOn(async {
    await asyncMain()
  })
}

async fn asyncMain(): Result<()> {
  // Set up file logging (silent failure is OK - logging is optional)
  let _logDir = setupFileLogging()

  let args = Cli.parse()

  match args.command {
    Command.config -> {
      // Force re-run wizard
      await runWizardFlow(&args)?
    }
    Command.log -> {
      showLastLog()?
    }
    null -> {
      // Check for --status flag first
      if args.status {
        await runStatus(&args)?
      } else if Config.exists() {
        // Subsequent run: use saved config
        await runWithConfig(&args)?
      } else if args.yes {
        // No config but -y flag: run with all detected managers
        await runWithoutConfig(&args)?
      } else {
        // First run: show wizard
        await runWizardFlow(&args)?
      }
    }
  }

  Ok(())
}

async fn runWizardFlow(args: &Cli): Result<()> {
  let report = scan()
  let (config, shouldExecute) = runWizard(report)?

  if shouldExecute {
    config.save()?
    if let path = Config.path() {
      println!("\nConfiguration saved to ${path.display()}\n")
    }

    let actions = getFilteredActions(config, report)
    if !actions.isEmpty() { await runActions(actions, args)? }
  } else {
    println!("Configuration cancelled.")
  }

  Ok(())
}

/// Run with all detected managers (no config, -y flag)
async fn runWithoutConfig(args: &Cli): Result<()> {
  // Scan and use detected managers
  let report = scan()
  let allActions = getActionsForScan(report)

  // Apply CLI filters
  let actions = filterActions(allActions, args.only.clone(), args.skip.clone())

  if actions.isEmpty() {
    println!("No actions to run.")
    return Ok(())
  }

  await runActions(actions, args)
}

/// Run status check - show outdated packages without updating
async fn runStatus(args: &Cli): Result<()> {
  let report = scan()
  let allActions = getCheckActionsForScan(report)

  // Apply CLI filters
  let actions = filterActions(allActions, args.only.clone(), args.skip.clone())

  if actions.isEmpty() {
    println!("No status checks available for detected managers.")
    return Ok(())
  }

  let needsSudo = !isWindows() && actions.iter().any { it.requiresPrivilege }

  // Refresh sudo credentials if needed
  await ensureSudoCredentials(needsSudo)?

  println!("Checking for outdated packages...\n")

  for action in actions.iter() {
    println!("[${action.manager}] ${action.description}")

    // Prepend sudo if needed
    let cmd = if action.requiresPrivilege && !isWindows() {
      sudo.withSudo(action.command)
    } else {
      action.command.clone()
    }

    let result = await executeCommand(cmd, OutputMode.streaming)
    match result {
      Ok(true) -> ()
      Ok(false) -> println!("  Error: Command failed\n")
      Err(e) -> println!("  Error: $e\n")
    }
    println!()
  }

  Ok(())
}

async fn runWithConfig(args: &Cli): Result<()> {
  let config = Config.load() ?? Config.default()
  let report = scan()

  // Build actions from config
  let actions = getFilteredActions(config, report)

  // Apply CLI overrides using filterActions
  let actions = filterActions(actions, args.only.clone(), args.skip.clone())

  if actions.isEmpty() {
    println!("No actions to run with current configuration.")
    println!("Use 'yup config' to reconfigure.")
    return Ok(())
  }

  // Show summary
  println!("Running ${actions.len()} action(s)...\n")

  await runActions(actions, args)
}

async fn runActions(actions: Array<Action>, args: &Cli): Result<()> {
  // Display planned actions
  println!("Planned actions (${actions.len()}):")
  var displayIndex: UIntSize = 1
  for action in actions.iter() {
    let sudoMarker = if action.requiresPrivilege { " [sudo]" } else { "" }
    println!("  ${displayIndex}. ${action.manager}: ${action.command} - ${action.description}${sudoMarker}")
    displayIndex += 1
  }
  println!()

  let needsSudo = !isWindows() && actions.iter().any { it.requiresPrivilege }

  // Show sudo warning if applicable
  if needsSudo {
    let privilegedCount = actions.iter().filter { it.requiresPrivilege }.count()
    println!("Note: ${privilegedCount} action(s) require sudo privileges.")
    println!()
  }

  // If dry-run, just show what would happen
  if args.dryRun {
    println!("[DRY RUN] Would execute the above actions.")
    return Ok(())
  }

  // Prompt for confirmation unless --yes is set
  if !args.yes {
    print!("Proceed with execution? [Y/n] ")
    io.stdout().flush()?

    var input = String.new()
    io.stdin().readLine(&var input)?

    if input.trim().eqIgnoreAsciiCase("n") {
      println!("Cancelled.")
      return Ok(())
    }
  }

  println!()

  // Refresh sudo credentials if needed
  await ensureSudoCredentials(needsSudo)?

  let total = actions.len()
  var successCount: UIntSize = 0
  var failedActions: Array<FailedAction> = []

  // Create progress bar
  let progress = ProgressBar.new(total as UInt64)
  progress.setStyle(
    ProgressStyle.defaultBar()
      .template("{spinner:.green} [{bar:25.cyan/dim}] {pos}/{len} {msg}")
      .expect("valid progress bar template")
      .progressChars("=> "),
  )
  progress.enableSteadyTick(Duration.fromMillis(100))

  var actionIndex: UIntSize = 1
  for action in actions.iter() {
    let start = Instant.now()

    // Determine the actual command to run (prepend sudo if needed)
    let cmdToRun = if action.requiresPrivilege && !isWindows() {
      sudo.withSudo(action.command)
    } else {
      action.command.clone()
    }

    // Update progress bar message with current action
    progress.setMessage("${action.command} - ${action.description}")

    // Execute command with progress integration
    let result = await executeCommand(
      cmdToRun,
      OutputMode.progress(progress: progress.clone(), verbose: args.verbose),
    )

    let elapsed = start.elapsed()
    match result {
      Ok(true) -> {
        successCount += 1
        if args.verbose { progress.suspend { println!("      -> Done ({:.1}s)", elapsed.asSecsF64()) } }
      }
      Ok(false) -> {
        failedActions.push(
          FailedAction(
            index: actionIndex,
            command: action.command.clone(),
            error: "Command failed".toString(),
          ),
        )
        if args.verbose { progress.suspend { println!("      -> FAILED ({:.1}s)", elapsed.asSecsF64()) } }
      }
      Err(e) -> {
        failedActions.push(
          FailedAction(index: actionIndex, command: action.command.clone(), error: e.toString()),
        )
        if args.verbose { progress.suspend { println!("      -> ERROR: $e") } }
      }
    }

    progress.inc(1)
    actionIndex += 1
  }

  // Finish progress bar
  if successCount == total {
    progress.finishWithMessage("All actions completed successfully")
  } else {
    progress.finishWithMessage("${successCount}/${total} succeeded, ${total - successCount} failed")
  }

  println!()
  println!("Completed: ${successCount}/${total} actions succeeded")

  // Show failed actions summary
  if !failedActions.isEmpty() {
    println!("\nFailed actions:")
    for failure in failedActions.iter() {
      println!("  ${failure.index}. ${failure.command} - ${failure.error}")
    }
  }

  Ok(())
}

/// Create a shell command for the current platform.
/// On Windows uses cmd.exe /C, on Unix uses sh -c.
fn createShellCommand(cmdStr: str): ProcessCommand {
  if isWindows() {
    var cmd = ProcessCommand.new("cmd")
    cmd.args(["/C", cmdStr])
    cmd
  } else {
    var cmd = ProcessCommand.new("sh")
    cmd.args(["-c", cmdStr])
    cmd
  }
}

enum OutputMode {
  case streaming
  case progress(progress: ProgressBar, verbose: Bool)
}

struct FailedAction(index: UIntSize, command: String, error: String)

/// - Progress: in verbose mode streams output above the progress bar;
///   otherwise captures stderr and only shows it on failure.
async fn executeCommand(cmdStr: str, mode: OutputMode): Result<Bool> {
  var cmd = createShellCommand(cmdStr)
  cmd.stdout(Stdio.piped())
  cmd.stderr(Stdio.piped())

  var child = cmd.spawn()?
  var progress: ProgressBar? = null
  var verbose = false
  var streamOutput = true

  match mode {
    OutputMode.streaming -> ()
    OutputMode.progress(progress: p, verbose: v) -> {
      progress = p
      verbose = v
      streamOutput = false
    }
  }

  let stdoutFuture = async {
    if let stdout = child.stdout.take() {
      let reader = BufReader.new(stdout)
      var lines = reader.lines()
      loop {
        let line = await lines.nextLine()?
        if line == null { break }
        let line = line!!
        if streamOutput {
          println!("      $line")
        } else if let progress = progress && verbose {
          progress.suspend { println!("      $line") }
        }
      }
    }
    Ok(())
  }

  let stderrFuture = async {
    var stderrOutput: Array<String> = []
    if let stderr = child.stderr.take() {
      let reader = BufReader.new(stderr)
      var lines = reader.lines()
      loop {
        let line = await lines.nextLine()?
        if line == null { break }
        let line = line!!
        if streamOutput {
          println!("      [err] $line")
        } else if let progress = progress {
          stderrOutput.push(line.clone())
          if verbose { progress.suspend { println!("      [err] $line") } }
        }
      }
    }
    Ok(stderrOutput)
  }

  async let stdoutResult = stdoutFuture()
  async let stderrResult = stderrFuture()
  let (stdoutResult, stderrResult) = await (stdoutResult, stderrResult)
  stdoutResult?
  let stderrOutput = stderrResult?

  let status = await child.wait()?

  // If command failed and not verbose, print stderr for debugging
  if !status.success() && !verbose && !stderrOutput.isEmpty() {
    if let progress = progress {
      progress.suspend {
        for line in stderrOutput.iter() {
          println!("      [err] $line")
        }
      }
    }
  }

  Ok(status.success())
}

async fn ensureSudoCredentials(needsSudo: Bool): Result<()> {
  if !needsSudo { return Ok(()) }

  if !sudo.isSudoAvailable() { return Err(anyhow!("Sudo is required but not available on this system")) }

  if !await sudo.hasValidCredentials() {
    println!("Refreshing sudo credentials...")
    if !await sudo.refreshCredentials()? { return Err(anyhow!("Failed to obtain sudo credentials")) }
    println!()
  }

  Ok(())
}

fn showLastLog(): Result<()> {
  guard let projDirs = ProjectDirs.from("", "", "yup") else {
    return Err(anyhow!("Cannot determine log directory"))
  }
  let logDir = projDirs.dataLocalDir()

  // Find the most recent log file
  var logFiles: Array<std.fs.DirEntry> = []
  for entryResult in std.fs.readDir(logDir)? {
    if let entry = entryResult.ok() {
      let path = entry.path()
      let pathText = path.toStringLossy()
      let isLog = pathText.endsWith(".log")
      if isLog { logFiles.push(entry) }
    }
  }

  guard !logFiles.isEmpty() else {
    println!("No log files found in ${logDir.display()}")
    return Ok(())
  }

  var latestIndex: UIntSize = 0
  var latestTime: std.time.SystemTime? = null
  var currentIndex: UIntSize = 0
  for entry in logFiles.iter() {
    let modified = entry.metadata().andThen { it.modified() }.ok()
    if let time = modified {
      if latestTime == null || time > latestTime!! {
        latestTime = time
        latestIndex = currentIndex
      }
    }
    currentIndex += 1
  }

  let latestPath = logFiles[latestIndex].path()
  println!("=== ${latestPath.display()} ===\n")

  let content = std.fs.readToString(latestPath)?
  println!(content)

  Ok(())
}
