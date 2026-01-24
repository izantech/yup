# Development Guide

## Oxide Migration Status

This project has been fully migrated to a hybrid Rust/Oxide codebase. The Oxide transpiler converts `.ox` files to Rust in the `oxide-gen/` directory.

**Migration Complete:**
- ✅ Phase 1: Infrastructure setup
- ✅ Phase 2: Core types and engine modules
- ✅ Phase 3: All 20 manager implementations converted to Oxide
- ✅ Phase 4: Core business logic (scan.ox, prompt.ox, sudo.ox, config.ox, filter.ox)
- ✅ Phase 5: CLI & Main kept in Rust (complex macro support)
- ✅ Phase 6: Testing & Verification

**Build Commands:**
```bash
cargo oxide build              # Transpile + debug build
cargo oxide build --release    # Transpile + release build
cargo oxide test               # Run tests
cargo oxide run -- --dry-run   # Test the binary
```

Source files (both `.ox` and `.rs`) live in `src/`. The Oxide transpiler generates `oxide-gen/` with the final Rust code.

## Project Structure

```
src/                          # Source files (Oxide + Rust)
├── main.rs                   # Entry point, CLI flow (RUST - tokio/clap macros)
├── cli.rs                    # Clap CLI definitions (RUST - derive macros)
├── config.ox                 # Config loading/saving (OXIDE)
├── prompt.ox                 # Interactive wizard (OXIDE)
├── sudo.ox                   # Sudo credential management (OXIDE)
└── engine/
    ├── mod.ox                # Engine module exports (OXIDE)
    ├── types.ox              # Core types: Manager, Action (OXIDE)
    ├── scan.ox               # System scanning, tool detection (OXIDE)
    ├── filter.ox             # Action filtering for --only/--skip (OXIDE)
    └── managers/
        ├── mod.ox            # PackageManager trait, create_manager() (OXIDE)
        └── *.ox              # 20 manager implementations (OXIDE)

oxide-gen/                    # Generated output (do not edit)
└── src/
    ├── main.rs               # Copied from src/
    ├── cli.rs                # Copied from src/
    ├── oxide_helpers.rs      # Oxide runtime helpers
    └── *.rs                  # Transpiled from *.ox files
```

### Files Kept in Rust

Two files in `src/` remain in Rust due to Oxide transpiler limitations with complex macros:
- **main.rs**: `#[tokio::main]` async runtime and clap derives
- **cli.rs**: Clap command-line argument derives

## Toolchain

The repo pins a Rust toolchain via `rust-toolchain.toml`. With rustup installed,
`cargo build` and `cargo clippy` will use the pinned version automatically.

## Architecture Overview

### Core Flow

1. **CLI parsing** (`cli.rs`): Parse arguments with clap
2. **Mode dispatch** (`main.rs`): Route to wizard, config-based run, or status check
3. **System scan** (`scan.ox`): Detect installed tools via `which`
4. **Action generation** (`managers/*.ox`): Create update/upgrade/check actions
5. **Execution** (`main.rs`): Run actions with progress bar

### Key Types

```oxide
// src/engine/types.ox
public enum Manager { Brew, Npm, Cargo, ... }  // All supported managers
public struct Action { manager, command, description, requires_privilege }
public struct ScanReport { available_managers, actionable_managers }
```

### The PackageManager Trait

```oxide
// src/engine/managers/mod.ox
public trait PackageManager {
    fn update_actions(): Vec<Action>      // Update package index
    fn upgrade_actions(): Vec<Action>     // Upgrade packages
    fn check_actions(): Vec<Action>       // Check for outdated (optional)
}
```

## Adding a New Package Manager

### Step 1: Add Manager Variant

In `src/engine/types.ox`, add to the `Manager` enum:

```oxide
public enum Manager {
    // ... existing variants
    MyManager
}
```

### Step 2: Create Implementation

Create `src/engine/managers/mymanager.ox`:

```oxide
//! MyManager package manager

import super.{Action, Manager, PackageManager}

/// MyManager package manager
public struct MyManagerManager

extension MyManagerManager: PackageManager {
    fn update_actions(): Vec<Action> {
        vec![Action {
            manager: Manager.MyManager,
            command: "mymanager refresh",
            description: "Refresh MyManager index",
            requires_privilege: false
        }]
    }

    fn upgrade_actions(): Vec<Action> {
        vec![Action {
            manager: Manager.MyManager,
            command: "mymanager upgrade --all",
            description: "Upgrade MyManager packages",
            requires_privilege: false
        }]
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
public fn create_manager(manager: &Manager): Box<dyn PackageManager>? {
    match manager {
        // ... existing cases
        Manager.MyManager -> Some(Box(MyManagerManager) as Box<dyn PackageManager>)
        else -> None
    }
}
```

### Step 4: Build

```bash
cargo oxide build
```

### Step 5: Update Documentation

- `README.md`: Add to supported managers list
- `INTERNALS.md`: Document commands
- `CHANGELOG.md`: Note the addition

## Platform Support

**Note:** Due to current Oxide codegen limitations, platform-specific `@[cfg(...)]`
attributes on modules and imports are not fully supported. All managers are compiled
on all platforms, and runtime detection (via `which`) handles availability.

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
| `oxide-stdlib` | Oxide standard library |

## Oxide Syntax Quick Reference

| Rust | Oxide |
|------|-------|
| `let mut x = 0;` | `var x = 0` |
| `fn foo() -> T` | `fn foo(): T` |
| `pub fn` | `public fn` |
| `#[derive(...)]` | `@[derive(...)]` |
| `impl Trait for T` | `extension T: Trait` |
| `match x { _ => }` | `match x { else -> }` |
| `\|x\| expr` | `{ x -> expr }` |
| `use crate::mod` | `import crate.mod` |
| `mod foo;` | `external module foo` |
| `Some(x).await` | `await Some(x)` |
| `format!("{}", x)` | `"$x"` |
