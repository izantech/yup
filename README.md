# yup

Safe, cross-platform updater for development tools.

`yup` detects how each tool was installed (Homebrew, asdf, nvm, pyenv, rbenv, conda, etc.) and updates using the correct manager. On first run, it shows an interactive configuration wizard to select which managers to update. Subsequent runs execute directly with saved preferences.

## Installation

### Cargo (recommended)

```bash
cargo install yup
```

### Homebrew (macOS/Linux)

```bash
brew tap izantech/tap
brew install yup
```

### winget (Windows)

```powershell
winget install izantech.yup
```

### Chocolatey (Windows)

```powershell
choco install yup
```

### From source

```bash
git clone https://github.com/izantech/yup
cd yup
cargo install --path .
```

## Usage

```bash
yup [OPTIONS] [COMMAND]
```

### Commands

| Command | Description |
|---------|-------------|
| `config` | Re-run the configuration wizard |
| `log` | Show the last run log |

### Options

| Flag | Description |
|------|-------------|
| `-h, --help` | Show help message |
| `-V, --version` | Show version |
| `-n, --dry-run` | Preview commands without executing |
| `-y, --yes` | Skip prompts, use saved config defaults |
| `-v, --verbose` | Show command output during execution |
| `--only <managers>` | Only update specified managers (comma-separated) |
| `--skip <managers>` | Skip specified managers (comma-separated) |
| `--status` | Check for outdated packages without updating |

### Examples

```bash
# First run: interactive configuration wizard
yup

# Subsequent runs: execute with saved config
yup

# Preview what would happen
yup --dry-run

# Non-interactive with all detected managers
yup --yes

# Re-configure which managers to update
yup config

# Only update Homebrew and npm
yup --only brew,npm

# Skip slow managers
yup --skip softwareupdate,mas

# Check for outdated packages
yup --status

# View last run log
yup log
```

## Configuration

On first run, `yup` shows an interactive wizard to:
1. Select which detected managers to update
2. Preview the commands that will run
3. Save configuration and execute

Configuration is stored at:
- **macOS:** `~/Library/Application Support/yup/config.toml`
- **Linux:** `~/.config/yup/config.toml`
- **Windows:** `%APPDATA%/yup/config.toml`

Run `yup config` to reconfigure at any time.

## Supported Managers

### System Package Managers
- **brew** - Homebrew (macOS/Linux)
- **apt** - Debian/Ubuntu
- **dnf** - Fedora/RHEL
- **pacman** - Arch Linux
- **winget** - Windows Package Manager
- **choco** - Chocolatey (Windows)
- **scoop** - Scoop (Windows)
- **port** - MacPorts
- **flatpak** - Flatpak (Linux)
- **snap** - Snap (Linux)
- **mas** - Mac App Store CLI
- **softwareupdate** - macOS system updates

### Version Managers
- **asdf** - Multiple runtime versions
- **mise** - Modern polyglot runtime manager (formerly rtx)
- **pyenv** - Python versions
- **rbenv** - Ruby versions
- **rvm** - Ruby Version Manager
- **nvm** - Node.js versions
- **fnm** - Fast Node Manager
- **volta** - JavaScript tool manager
- **conda/mamba** - Python environments
- **sdkman** - Java ecosystem

### Language Package Managers
- **npm** - Node.js packages
- **pnpm** - Fast Node.js package manager
- **yarn** - Node.js packages
- **bun** - JavaScript runtime & package manager
- **pip** - Python packages
- **pipx** - Python CLI apps
- **poetry** - Python dependency management
- **uv** - Fast Python package installer
- **gem** - Ruby gems
- **rustup** - Rust toolchains
- **cargo** - Rust packages (via cargo-update)
- **go** - Go modules
- **composer** - PHP packages
- **helm** - Kubernetes packages
- **krew** - kubectl plugins

## Log Files

Logs are stored in platform-appropriate directories:
- **macOS:** `~/Library/Application Support/yup/`
- **Linux:** `~/.local/share/yup/`
- **Windows:** `%APPDATA%/yup/`

Daily rotation with filenames like `yup.2026-01-11.log`.

## Privilege Handling

Some package managers require root/admin privileges (sudo) to run updates:

- **Linux:** apt, dnf, pacman, snap
- **macOS:** port, mas, softwareupdate

When privileged commands are queued, `yup` will:
1. Show `[sudo]` markers next to commands that need elevation
2. Prompt for your password once before execution (via `sudo -v`)
3. Automatically prepend `sudo` to those commands

This means you only enter your password once, not for each command.

## Requirements

- macOS, Linux, or Windows

## License

[MIT](LICENSE)
