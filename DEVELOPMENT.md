# Development Guide

## Oxide Migration Status

This project has been migrated from Rust to a hybrid Rust/Oxide codebase. The build uses `cargo oxide` which transpiles `.ox` files to Rust and builds from `oxide-gen/`.

**Completed:**
- Phase 1: Infrastructure (experimental/oxide branch)
- Phase 2: Core types and engine modules
- Phase 3: Manager implementations (20 managers converted to Oxide)
- Phase 4: Core business logic (scan.ox, prompt.ox, sudo.ox)
- Phase 5: CLI & Main (kept in Rust for complex macro support)

**Build Commands:**
- `cd oxide-gen && cargo build` - Build directly (recommended)
- `cd oxide-gen && cargo test` - Run tests
- `cd oxide-gen && cargo run -- --dry-run` - Test the binary

**Note:** Due to the hybrid Rust/Oxide structure, build directly from `oxide-gen/` rather than using `cargo oxide build` from the root. The root `cargo oxide` command would regenerate oxide-gen and overwrite the manually-maintained Rust files.

## Project Structure

```
src/                          # Oxide source files
├── prompt.ox                 # Interactive wizard (OXIDE)
├── sudo.ox                   # Sudo credential management (OXIDE)
└── engine/
    ├── scan.ox               # System scanning, tool detection (OXIDE)
    └── managers/
        ├── mod.ox            # PackageManager trait, create_manager() (OXIDE)
        └── *.ox              # 20 manager implementations (OXIDE)

oxide-gen/                    # Generated/maintained Rust files
└── src/
    ├── main.rs               # Entry point, CLI flow (RUST - complex macros)
    ├── cli.rs                # Clap CLI definitions (RUST - derive macros)
    ├── config.rs             # Config loading/saving (RUST - complex types)
    └── engine/
        ├── mod.rs            # Engine module (RUST - maintained)
        ├── types.rs          # Core types (RUST - complex derives)
        ├── filter.rs         # Action filtering (RUST - complex types)
        └── ...               # Transpiled from *.ox files
```

### Files Kept in Rust

Some files are kept in Rust due to Oxide transpiler limitations:
- **main.rs, cli.rs**: Complex macro attributes (`#[tokio::main]`, clap derives)
- **types.rs**: Complex derive macros (strum, serde), public struct fields
- **filter.rs**: Complex generic types (`Option<&[String]>`)
- **config.rs**: Closure syntax for `ok_or_else`

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

```rust
// oxide-gen/src/engine/types.rs
enum Manager { Brew, Npm, Cargo, ... }  // All supported managers
struct Action { manager, command, description, requires_privilege }
struct ScanReport { available_managers, actionable_managers }
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

In `oxide-gen/src/engine/types.rs`, add to the `Manager` enum:

```rust
pub enum Manager {
    // ... existing variants
    MyManager,
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
public fn create_manager(manager: &Manager): Box<dyn PackageManager>? {
    match manager {
        // ... existing cases
        Manager.MyManager -> Some(Box.new(MyManagerManager) as Box<dyn PackageManager>)
    }
}
```

### Step 4: Update Documentation

- `README.md`: Add to supported managers list
- `CHANGELOG.md`: Note the addition

## Platform Support

**Note:** Due to current Oxide codegen limitations, platform-specific `@[cfg(...)]`
attributes on modules and imports are not fully supported. All managers are compiled
on all platforms, and runtime detection (via `which`) handles availability.

## Testing

### Unit Tests

```bash
cd oxide-gen && cargo test
```

### Manual Testing

```bash
cd oxide-gen
cargo run -- --dry-run      # Preview actions without executing
cargo run -- --status       # Check for outdated packages
cargo run -- --verbose      # See command output
cargo run -- --only brew    # Test specific manager
```

## Build & Release

### Local Build

```bash
cd oxide-gen && cargo build --release
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
