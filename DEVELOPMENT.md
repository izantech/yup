# Development Guide

## Oxide Migration Status

This project is being migrated from Rust to Oxide syntax. The build uses `cargo oxide` instead of `cargo`.

**Completed:**
- Phase 1: Infrastructure (experimental/oxide branch)
- Phase 2: Core types and engine modules
- Phase 3: Manager implementations (20 managers converted)

**In Progress:**
- Phase 4: Core business logic (scan.ox, filter.ox, config.ox, prompt.ox, sudo.ox)

**Build Command:** `cargo oxide build`

## Project Structure

```
src/
├── main.rs           # Entry point, CLI flow, command execution (pending migration)
├── cli.rs            # Clap CLI argument definitions (pending migration)
├── config.rs         # Config loading/saving (TOML) (pending migration)
├── prompt.rs         # Interactive wizard (dialoguer) (pending migration)
├── sudo.rs           # Sudo credential management (Unix only) (pending migration)
└── engine/
    ├── mod.ox        # Module exports (MIGRATED)
    ├── types.ox      # Core types: Manager, Action, ScanReport (MIGRATED)
    ├── scan.rs       # System scanning, tool detection (pending migration)
    ├── filter.rs     # Action filtering (--only/--skip) (pending migration)
    └── managers/
        ├── mod.ox    # PackageManager trait, create_manager() factory (MIGRATED)
        └── *.ox      # Individual manager implementations (MIGRATED - 20 files)
```

## Toolchain

The repo pins a Rust toolchain via `rust-toolchain.toml`. With rustup installed,
`cargo build` and `cargo clippy` will use the pinned version automatically.

## Architecture Overview

### Core Flow

1. **CLI parsing** (`cli.rs`): Parse arguments with clap
2. **Mode dispatch** (`main.rs`): Route to wizard, config-based run, or status check
3. **System scan** (`scan.rs`): Detect installed tools via `which`
4. **Provenance detection** (`detect.rs`): Determine which manager installed each tool
5. **Action generation** (`managers/*.rs`): Create update/upgrade/check actions
6. **Execution** (`main.rs`): Run actions with progress bar

### Key Types

```rust
// src/engine/types.rs
enum Manager { Brew, Npm, Cargo, ... }  // All supported managers
struct Action { manager, command, description, requires_privilege }
struct ScanReport { detected_tools, available_managers, actionable_managers }
```

### The PackageManager Trait

```rust
// src/engine/managers/mod.rs
pub trait PackageManager {
    fn update_actions(&self) -> Vec<Action>;      // Update package index
    fn upgrade_actions(&self) -> Vec<Action>;     // Upgrade packages
    fn check_actions(&self) -> Vec<Action>;       // Check for outdated (optional; default impl)
}
```

Note: `check_actions()` has a default implementation; managers can override it.

## Adding a New Package Manager

### Step 1: Add Manager Variant

In `src/engine/types.rs`, add to the `Manager` enum:

```rust
pub enum Manager {
    // ... existing variants
    MyManager,
}
```

Update the `FromStr` and `as_str`/`display_name` implementations in the same file.

### Step 2: Create Implementation

Create `src/engine/managers/mymanager.ox`:

```oxide
//! MyManager package manager

import super.{Action, Manager, PackageManager}

/// MyManager package manager
public struct MyManagerManager

extension MyManagerManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action.new(
            Manager.MyManager,
            "mymanager refresh",
            "Refresh MyManager index",
            false
        )]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action.new(
            Manager.MyManager,
            "mymanager upgrade --all",
            "Upgrade MyManager packages",
            false
        )]
    }

    fn check_actions(): Vec<Action> {
        vec![]
    }
}
```

### Step 3: Register in mod.ox

In `src/engine/managers/mod.ox`:

```oxide
// Add module declaration
external module mymanager

// Add re-export
public import mymanager.MyManagerManager

// Add case in create_manager()
public fn create_manager(manager: Manager): Box<dyn PackageManager>? {
    match manager {
        // ... existing cases
        Manager.MyManager -> Some(Box.new(MyManagerManager) as Box<dyn PackageManager>)
    }
}
```

### Step 4: Add to Scanner

In `src/engine/scan.rs`, add to the `TOOLS` array if auto-detection is needed:

```rust
const TOOLS: &[&str] = &[
    // ... existing tools
    "mymanager",
];
```

### Step 5: Update Documentation

- `README.md`: Add to supported managers list
- `INTERNALS.md`: Document the commands used
- `CHANGELOG.md`: Note the addition

## Platform Support

**Note:** Due to current Oxide codegen limitations, platform-specific `@[cfg(...)]`
attributes on modules and imports are not fully supported. All managers are compiled
on all platforms, and runtime detection (via `which`) handles availability.

All managers are compiled cross-platform. The `scan.rs` file uses `which` to detect
which managers are actually installed and available on the current system.

## Testing

### Unit Tests

```bash
cargo oxide test
```

### Manual Testing

```bash
cargo oxide run -- --dry-run      # Preview actions without executing
cargo oxide run -- --status       # Check for outdated packages
cargo oxide run -- --verbose      # See command output
cargo oxide run -- --only brew    # Test specific manager
```

## Build & Release

### Local Build

```bash
cargo oxide build --release
```

### Release Process

1. **Publish to crates.io:**
```bash
cargo publish --dry-run  # Verify first
cargo publish
```

2. **Tag the release:**
```bash
git tag v0.1.0
git push origin v0.1.0
```

3. **Update Homebrew formula** in `izantech/homebrew-tap` with the new version and SHA256.

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `dialoguer` | Interactive prompts |
| `indicatif` | Progress bars |
| `tokio` | Async command execution |
| `which` | PATH-based tool detection |
| `directories` | Platform-appropriate config paths |
| `serde` / `toml` | Config serialization |
| `tracing` | Structured logging |
