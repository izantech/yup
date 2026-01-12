# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## Build & Test Commands

```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo test               # Run all tests
cargo test <test_name>   # Run a specific test
cargo fmt                # Format code
cargo clippy             # Lint
cargo run -- --dry-run   # Preview actions without executing
cargo run -- --verbose   # Run with command output visible
cargo run -- --only brew # Test specific manager
```

## Release Process

Uses `cargo-release` for version bumps:
```bash
cargo release patch --execute  # 0.1.0 → 0.1.1
cargo release minor --execute  # 0.1.1 → 0.2.0
```

## Architecture

**Core flow:** CLI parsing → Mode dispatch → System scan → Provenance detection → Action generation → Execution

### Key Files

- `src/main.rs` - Entry point, command execution loop with progress bar
- `src/cli.rs` - Clap CLI argument definitions
- `src/config.rs` - TOML config loading/saving
- `src/prompt.rs` - Interactive wizard (dialoguer)
- `src/sudo.rs` - Unix sudo credential management

### Engine Module (`src/engine/`)

- `types.rs` - Core types: `Manager` enum, `Action`, `DetectedTool`, `ScanReport`
- `scan.rs` - System scanning, tool detection via `which`
- `detect.rs` - Path-based manager detection (determines which manager installed a tool)
- `filter.rs` - Action filtering for `--only`/`--skip` flags
- `managers/mod.rs` - `PackageManager` trait and `create_manager()` factory
- `managers/*.rs` - Individual manager implementations

### The PackageManager Trait

```rust
pub trait PackageManager {
    fn update_actions(&self) -> Vec<Action>;   // Refresh package index
    fn upgrade_actions(&self) -> Vec<Action>;  // Upgrade packages
    fn check_actions(&self) -> Vec<Action>;    // Check for outdated (optional; default impl)
}
```

Note: `check_actions()` has a default implementation; managers can override it to add checks.

### Platform-Specific Compilation

Managers use conditional compilation:
- macOS: `#[cfg(target_os = "macos")]` - brew, port, mas, softwareupdate
- Linux: `#[cfg(target_os = "linux")]` - apt, dnf, pacman, flatpak, snap
- Windows: `#[cfg(target_os = "windows")]` - winget, choco, scoop
- Cross-platform (no `#[cfg]`): npm, pnpm, cargo, rustup, gem, pipx, mise, conda

## Adding a New Package Manager

1. Add variant to `Manager` enum in `src/engine/types.rs`
2. Update `FromStr` implementation in the same file
3. Create `src/engine/managers/<name>.rs` implementing `PackageManager`
4. Register in `src/engine/managers/mod.rs` (module declaration + `create_manager()` match arm)
5. Add tool name to `TOOLS` array in `src/engine/scan.rs` for auto-detection
6. Update docs: README.md (supported managers), INTERNALS.md (commands), CHANGELOG.md

## Code Style

- Error handling: `anyhow::Result`
- Logging: `tracing` macros (`debug!`, `info!`)
- Conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`

## Toolchain

This repo pins a Rust toolchain via `rust-toolchain.toml`. With rustup installed,
`cargo build` and `cargo clippy` will use the pinned version automatically.
