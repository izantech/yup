use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "yup",
    version,
    about = "Safe, cross-platform updater for development tools",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Preview commands without executing
    #[arg(short = 'n', long = "dry-run", global = true)]
    pub dry_run: bool,

    /// Skip prompts, use saved config defaults
    #[arg(short = 'y', long, global = true)]
    pub yes: bool,

    /// Only update specified managers (comma-separated)
    #[arg(long, value_delimiter = ',', global = true)]
    pub only: Option<Vec<String>>,

    /// Skip specified managers (comma-separated)
    #[arg(long, value_delimiter = ',', global = true)]
    pub skip: Option<Vec<String>>,

    /// Show outdated packages without updating
    #[arg(short = 's', long = "status", global = true)]
    pub status: bool,

    /// Show command output during execution
    #[arg(short = 'v', long = "verbose", global = true)]
    pub verbose: bool,

    /// Pass --greedy to brew upgrade (include auto-updating casks)
    #[arg(long, global = true)]
    pub greedy: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Re-run the configuration wizard
    Config,
    /// Show the last run log
    Log,
}
