import clap.{ Parser, Subcommand }

@[derive(Parser , Debug)]
@[command(
  name = "yup",
  version,
  about = "Safe, cross-platform updater for development tools",
  long_about = null
)]
public struct Cli(
  @[command(subcommand)]
  command: Command?,

  /// Preview commands without executing
  @[arg(short = 'n', long = "dry-run", global = true)]
  dryRun: Bool,

  /// Skip prompts, use saved config defaults
  @[arg(short = 'y', long, global = true)]
  yes: Bool,

  /// Only update specified managers (comma-separated)
  @[arg(long, value_delimiter = ',', global = true)]
  only: Array<String>?,

  /// Skip specified managers (comma-separated)
  @[arg(long, value_delimiter = ',', global = true)]
  skip: Array<String>?,

  /// Show outdated packages without updating
  @[arg(short = 's', long = "status", global = true)]
  status: Bool,

  /// Show command output during execution
  @[arg(short = 'v', long = "verbose", global = true)]
  verbose: Bool,
)

@[derive(Subcommand , Debug)]
public enum Command {
  /// Re-run the configuration wizard
  case config
  /// Show the last run log
  case log
}
