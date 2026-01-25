# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## Build & Test Commands

This project uses Oxide (transpiled to Rust). Use `cargo oxide` from the project root:

```bash
cargo oxide build              # Transpile + debug build
cargo oxide build --release    # Transpile + release build
cargo oxide test               # Run all tests
cargo oxide test <test_name>   # Run a specific test
cargo oxide run -- --dry-run   # Preview actions without executing
cargo oxide run -- --verbose   # Run with command output visible
cargo oxide run -- --only brew # Test specific manager
cargo oxide fmt                # Format generated code
cargo oxide clippy             # Lint generated code
```

## Release Process

Uses `cargo-release` for version bumps:
```bash
cargo release patch --execute  # 0.1.0 → 0.1.1
cargo release minor --execute  # 0.1.1 → 0.2.0
```

## Architecture

**Core flow:** CLI parsing → Mode dispatch → System scan → Action generation → Execution

### Key Files

All source files are in `src/` as Oxide `.ox` files (100% Oxide migration complete):

- `src/main.ox` - Entry point, command execution loop with progress bar
- `src/cli.ox` - Clap CLI argument definitions
- `src/config.ox` - TOML config loading/saving
- `src/prompt.ox` - Interactive wizard using dialoguer
- `src/sudo.ox` - Unix sudo credential management

### Engine Module (`src/engine/`)

- `types.ox` - Core types: `Manager` enum, `Action`, `DetectedTool`, `ScanReport`
- `scan.ox` - System scanning, tool detection via `which`
- `filter.ox` - Action filtering for `--only`/`--skip` flags
- `managers/mod.ox` - `PackageManager` trait and `create_manager()` factory
- `managers/*.ox` - Individual manager implementations (20 managers)

### The PackageManager Trait

```oxide
public trait PackageManager {
    fn update_actions(): Vec<Action>   // Refresh package index
    fn upgrade_actions(): Vec<Action>  // Upgrade packages
    fn check_actions(): Vec<Action>    // Check for outdated (optional; default impl)
}
```

Note: `check_actions()` has a default implementation; managers can override it to add checks.

### Platform-Specific Compilation

Due to Oxide codegen limitations, all managers compile on all platforms. Runtime detection via `which` handles availability:
- macOS-specific: brew, port, mas, softwareupdate
- Linux-specific: apt, dnf, pacman, flatpak, snap
- Windows-specific: winget, choco, scoop
- Cross-platform: npm, pnpm, cargo, rustup, gem, pipx, mise, conda

## Adding a New Package Manager

1. Add variant to `Manager` enum in `src/engine/types.ox`
2. Create `src/engine/managers/<name>.ox` implementing `PackageManager`
3. Register in `src/engine/managers/mod.ox` (module declaration + `create_manager()` match arm)
4. Add tool name to `TOOLS` array in `src/engine/scan.ox` for auto-detection
5. Build with `cargo oxide build`
6. Update docs: README.md (supported managers), INTERNALS.md (commands), CHANGELOG.md

See DEVELOPMENT.md for detailed Oxide syntax examples.

## Code Style

- Error handling: `anyhow::Result`
- Logging: `tracing` macros (`debug!`, `info!`)
- Conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`

## Toolchain

This repo pins a Rust toolchain via `rust-toolchain.toml`. With rustup installed,
`cargo build` and `cargo clippy` will use the pinned version automatically.
