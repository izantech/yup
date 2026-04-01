# yup

Safe, cross-platform updater that upgrades packages across your installed managers.

`yup` detects your installed package managers and updates them all with a single command. On first run, it shows an interactive configuration wizard to select which managers to update. Subsequent runs execute directly with saved preferences.

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
| `--greedy` | Pass `--greedy` to brew upgrade (include auto-updating casks) |

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

# Include auto-updating casks in brew upgrade
yup --greedy

# View last run log
yup log
```

## Configuration

On first run, `yup` shows an interactive wizard to:
1. Select which detected managers to update
2. Configure manager-specific options (brew `--greedy`, mise `--yes`)
3. Preview the commands that will run
4. Save configuration and execute

Re-running `yup config` loads your existing config and pre-selects your previously enabled managers.

Configuration is stored at:
- **macOS:** `~/Library/Application Support/yup/config.toml`
- **Linux:** `~/.config/yup/config.toml`
- **Windows:** `%APPDATA%/yup/config.toml`

Run `yup config` to reconfigure at any time.

### Brew Greedy Mode

Add a `[brew]` section to your config to always pass `--greedy` to brew upgrade commands:

```toml
[brew]
greedy = true
```

This includes auto-updating casks in the upgrade. The `--greedy` CLI flag overrides this setting per-run.

### Mise Self-Update Confirmation

By default, `yup` passes `--yes` to `mise self-update` to skip the confirmation prompt. You can disable this in the `[mise]` config section:

```toml
[mise]
yes = false
```

### Full Config Example

```toml
enabled_managers = ["brew", "mise", "cargo"]

[brew]
greedy = false

[mise]
yes = true
```

## Supported Managers

`yup` focuses on package managers that support global package upgrades. Currently supporting **20** managers across platforms.

### System Package Managers
- **brew** - Homebrew (macOS/Linux)
- **port** - MacPorts (macOS)
- **mas** - Mac App Store CLI (macOS)
- **softwareupdate** - macOS updates
- **apt** - Debian/Ubuntu (Linux)
- **dnf** - Fedora/RHEL (Linux)
- **pacman** - Arch Linux
- **flatpak** - Flatpak (Linux)
- **snap** - Snap (Linux)
- **winget** - Windows Package Manager
- **choco** - Chocolatey (Windows)
- **scoop** - Scoop (Windows)

### Version Managers
- **mise** - Modern polyglot runtime manager (formerly rtx)
- **conda/mamba** - Python/data science environments

### Language Package Managers
- **npm** - Global Node.js packages
- **pnpm** - Global Node.js packages (auto-detects corepack)
- **pipx** - Python CLI applications (isolated environments)
- **gem** - Ruby gems
- **rustup** - Rust toolchains
- **cargo** - Rust packages (via cargo-update)

## Log Files

Logs are stored in platform-appropriate directories:
- **macOS:** `~/Library/Application Support/yup/`
- **Linux:** `~/.local/share/yup/`
- **Windows:** `%APPDATA%/yup/`

Daily rotation with filenames like `yup.2026-01-11.log`.

## Toolchain

This repo pins a Rust toolchain via `rust-toolchain.toml`. With rustup installed,
`cargo build` and `cargo clippy` will use the pinned version automatically.

## Privilege Handling

Some package managers require root/admin privileges (sudo) to run updates:

- **Linux:** apt, dnf, pacman, snap
- **macOS:** port, mas, softwareupdate

When privileged commands are queued, `yup` will:
1. Show `[sudo]` markers next to commands that need elevation
2. Prompt for your password once before execution (via `sudo -v`)
3. Automatically prepend `sudo` to those commands

This means you only enter your password once, not for each command.

## Development

A `./dev` script is provided for common workflows:

```bash
./dev build              # Debug build (--release for release)
./dev test               # Run all tests
./dev run -- --dry-run   # Build and run with args
./dev check              # fmt + clippy + tests
./dev fmt                # Format code
./dev                    # Default: check + build
```

## Requirements

- macOS, Linux, or Windows

## License

[MIT](LICENSE)
