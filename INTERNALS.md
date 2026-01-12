# Internals

This document describes the commands executed by `yup` for each supported package manager.

## Command Types

- **Update** - Refresh package metadata/indices (e.g., `brew update`)
- **Upgrade** - Install newer versions of packages (e.g., `brew upgrade`)
- **Check** - List outdated packages without making changes (used by `--status`)

## Supported Package Managers

### System Package Managers

| Manager | Commands |
|---------|----------|
| **brew** | `brew update`<br>`brew upgrade`<br>`brew upgrade --cask`<br>`brew outdated`<br>`brew outdated --cask` |
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
| **asdf** | `asdf plugin update --all` |
| **mise** | `mise self-update`<br>`mise plugins update`<br>`mise upgrade`<br>`mise outdated` |
| **pyenv** | `pyenv update` |
| **rbenv** | `rbenv rehash` |
| **rvm** | `bash -c 'source "${rvm_path:-$HOME/.rvm}/scripts/rvm" && rvm get stable'` |
| **nvm** | `bash -c 'export NVM_DIR="${NVM_DIR:-$HOME/.nvm}" && . "$NVM_DIR/nvm.sh" && nvm install-latest-npm'` |
| **fnm** | - |
| **volta** | `volta setup` |
| **conda** | `conda update -n base conda -y`<br>`conda update -n base --all -y` |
| **sdkman** | `bash -c 'source "${SDKMAN_DIR:-$HOME/.sdkman}/bin/sdkman-init.sh" && sdk selfupdate'`<br>`bash -c 'source "${SDKMAN_DIR:-$HOME/.sdkman}/bin/sdkman-init.sh" && sdk update'` |

### Language Package Managers

| Manager | Commands |
|---------|----------|
| **npm** | `npm update -g`<br>`npm outdated -g` |
| **pnpm** | `pnpm self-update`<br>`pnpm update -g`<br>`pnpm outdated -g` |
| **yarn** | `yarn set version stable` |
| **bun** | `bun upgrade` |
| **pip** | `pip install --upgrade pip`<br>`pip list --outdated` |
| **pipx** | `pipx upgrade-all` |
| **poetry** | `poetry self update` |
| **uv** | `uv self update` |
| **gem** | `gem update --system`<br>`gem update`<br>`gem outdated` |
| **rustup** | `rustup update`<br>`rustup check` |
| **cargo** | `cargo install-update -a`<br>`cargo install-update -l` |
| **go** | - |
| **composer** | `composer self-update`<br>`composer global update` |
| **helm** | `helm repo update` |
| **krew** | `kubectl krew update`<br>`kubectl krew upgrade` |
