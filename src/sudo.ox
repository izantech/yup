//! Sudo credential management for privileged commands

import tokio.process.Command as ProcessCommand
import which.which

/// Check if sudo is available on this system
public fn is_sudo_available(): bool {
    which("sudo").is_ok()
}

/// Check if the current user already has valid sudo credentials
/// (e.g., from a recent sudo invocation within the timeout period)
public async fn has_valid_credentials(): bool {
    // Use output() which captures all streams, avoiding keyword conflicts
    let result = await ProcessCommand.new("sudo")
        .args(vec!["-n", "true"])
        .output()

    result.map({ output -> output.status.success() }).unwrap_or(false)
}

/// Refresh sudo credentials by prompting the user
/// Returns true if credentials were successfully obtained
public async fn refresh_credentials(): anyhow.Result<bool> {
    let status = await ProcessCommand.new("sudo")
        .arg("-v")
        .status()
    let status = status?

    Ok(status.success())
}

/// Prepend sudo to a command string
public fn with_sudo(command: &str): String {
    "sudo $command"
}
