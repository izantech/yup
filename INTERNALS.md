# Internals

This document describes the commands executed by `yup` for each supported package manager.

## Command Types

- **Update** - Refresh package metadata/indices (e.g., `brew update`)
- **Upgrade** - Install newer versions of packages (e.g., `brew upgrade`)
- **Check** - List outdated packages without making changes (used by `--status`)

## Privilege Requirements

Some managers require root privileges. On Unix systems, `yup` automatically prepends `sudo` to these commands:

| Manager | Requires sudo |
|---------|---------------|
| apt, dnf, pacman, snap | Yes (Linux) |
| port, mas, softwareupdate | Yes (macOS) |
| choco | Yes (Windows - admin) |
| All others | No |

When privileged commands are queued, `yup` prompts for your password once via `sudo -v` before execution.

## Supported Package Managers

### System Package Managers

| Manager | Commands |
|---------|----------|
| **brew** | `brew update`<br>`brew upgrade`<br>`brew upgrade --cask`<br>`brew outdated`<br>`brew outdated --cask`<br>With `--greedy`: appends `--greedy` to upgrade commands |
| **apt** | `apt update`<br>`apt upgrade -y`<br>`apt autoremove -y` |
| **dnf** | `dnf upgrade -y`<br>`dnf autoremove -y` |
| **pacman** | `pacman -Syu --noconfirm` |
| **port** | `port selfupdate`<br>`port upgrade outdated` |
| **snap** | `snap refresh` |
| **flatpak** | `flatpak update -y` |
| **winget** | `winget upgrade --all --accept-package-agreements --accept-source-agreements` |
| **choco** | `choco upgrade all -y` |
| **scoop** | `scoop update`<br>`scoop update *` |
| **mas** | `mas upgrade`<br>`mas outdated` |
| **softwareupdate** | `softwareupdate -ia` |

### Version Managers

| Manager | Commands |
|---------|----------|
| **mise** | `mise self-update --yes`<br>`mise plugins update`<br>`mise upgrade`<br>`mise outdated` |
| **conda** | `conda update -n base conda -y`<br>`conda update -n base --all -y` |

### Language Package Managers

| Manager | Commands |
|---------|----------|
| **npm** | `npm update -g`<br>`npm outdated -g` |
| **pnpm** | `pnpm self-update` (or `corepack install -g pnpm@latest` if corepack is available)<br>`pnpm update -g`<br>`pnpm outdated -g` |
| **pipx** | `pipx upgrade-all` |
| **gem** | `gem update --system`<br>`gem update`<br>`gem outdated` |
| **rustup** | `rustup update`<br>`rustup check` |
| **cargo** | `cargo install-update -a`<br>`cargo install-update -l` |
