# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Windows ARM64 builds (aarch64-pc-windows-msvc)
- Chocolatey package distribution (`choco install yup`)
- winget package distribution (`winget install izantech.yup`)
- Windows shell command support for cross-platform execution
- `check_actions` for showing outdated packages via `--status` flag

### Changed

- Removed 17 package managers that lack meaningful global upgrade support:
  - **Project-local managers**: yarn, poetry, bun, uv, pip, go
  - **Version managers without bulk upgrade**: fnm, nvm, asdf, pyenv, rbenv, volta, rvm, sdkman
  - **Niche managers**: helm, krew, composer
- Simplified path detection logic for removed version managers
- Updated documentation to reflect focused manager support (20 managers)

### Fixed

- Windows path detection for package managers (Chocolatey, Scoop)
- Wrapped Unix-specific paths with conditional compilation to prevent dead code on Windows

## [0.1.0] - 2026-01-12

Initial release of yup - a safe, cross-platform updater for development tools.

### Features

- **Interactive Configuration Wizard** - First-run wizard to select which package managers to update
- **Cross-Platform Support** - Works on macOS, Linux, and Windows
- **35+ Package Managers** - Detects and updates tools from various ecosystems
- **Smart Detection** - Identifies how each tool was installed (Homebrew, mise, npm, etc.)
- **Sudo Support** - Automatically handles privilege elevation for system package managers
- **Progress Display** - Real-time progress bar during execution
- **Status Checks** - Check for outdated packages without updating (`--status`)
- **Dry Run Mode** - Preview commands without executing (`--dry-run`)
- **Selective Updates** - Filter managers with `--only` and `--skip`
- **Log Rotation** - Daily log files in platform-appropriate directories

### CLI Options

| Flag | Description |
|------|-------------|
| `-n, --dry-run` | Preview commands without executing |
| `-y, --yes` | Skip prompts, use saved config |
| `-v, --verbose` | Show command output during execution |
| `--only <managers>` | Only update specified managers |
| `--skip <managers>` | Skip specified managers |
| `--status` | Check for outdated packages |
| `config` | Re-run configuration wizard |
| `log` | Show last run log |

### Supported Package Managers

**System Package Managers:**
- brew (Homebrew)
- apt (Debian/Ubuntu)
- dnf (Fedora/RHEL)
- pacman (Arch Linux)
- winget (Windows)
- choco (Chocolatey)
- scoop (Windows)
- port (MacPorts)
- flatpak (Linux)
- snap (Linux)
- mas (Mac App Store)
- softwareupdate (macOS)

**Version Managers:**
- asdf
- mise (formerly rtx)
- pyenv
- rbenv
- rvm
- nvm
- fnm
- volta
- conda
- sdkman

**Language Package Managers:**
- npm, pnpm, yarn, bun (JavaScript)
- pip, pipx, poetry, uv (Python)
- gem (Ruby)
- rustup, cargo (Rust)
- go (Go)
- composer (PHP)
- helm, krew (Kubernetes)

### Privilege Handling

Commands from privileged managers (apt, dnf, pacman, snap, port, mas, softwareupdate) are automatically prefixed with `sudo`. The tool prompts for credentials once before execution, not for each command.
