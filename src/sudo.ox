//! Sudo credential management for privileged commands

import tokio.process.Command as ProcessCommand
import anyhow.Result
import which.which

/// Check if sudo is available on this system
public fn isSudoAvailable(): bool { which("sudo").isOk() }

/// Check if the current user already has valid sudo credentials
/// (e.g., from a recent sudo invocation within the timeout period)
public async fn hasValidCredentials(): bool {
  // Use output() which captures all streams, avoiding keyword conflicts
  let result = await ProcessCommand.new("sudo").args(["-n", "true"]).output()

  result.map {
    it.status.success()
  }.unwrapOr(false)
}

/// Refresh sudo credentials by prompting the user
/// Returns true if credentials were successfully obtained
public async fn refreshCredentials(): Result<bool> {
  let status = await ProcessCommand.new("sudo").arg("-v").status()
  let status = status?

  Ok(status.success())
}

/// Prepend sudo to a command string
public fn withSudo(command: str): String { "sudo $command" }
