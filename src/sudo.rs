//! Sudo credential management for privileged commands

use std::process::Stdio;
use tokio::process::Command as ProcessCommand;

/// Check if sudo is available on this system
pub fn is_sudo_available() -> bool {
    which::which("sudo").is_ok()
}

/// Check if the current user already has valid sudo credentials
/// (e.g., from a recent sudo invocation within the timeout period)
pub async fn has_valid_credentials() -> bool {
    let result = ProcessCommand::new("sudo")
        .args(["-n", "true"]) // -n = non-interactive, exit 1 if password needed
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;

    result.map(|s| s.success()).unwrap_or(false)
}

/// Refresh sudo credentials by prompting the user
/// Returns true if credentials were successfully obtained
pub async fn refresh_credentials() -> anyhow::Result<bool> {
    let status = ProcessCommand::new("sudo")
        .args(["-v"]) // Validate and extend sudo timeout
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    Ok(status.success())
}

/// Prepend sudo to a command string
pub fn with_sudo(command: &str) -> String {
    format!("sudo {}", command)
}
