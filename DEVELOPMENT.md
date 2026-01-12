# Development Guide

## Project Structure

```
src/
├── main.rs           # Entry point, CLI flow, command execution
├── cli.rs            # Clap CLI argument definitions
├── config.rs         # Config loading/saving (TOML)
├── prompt.rs         # Interactive wizard (dialoguer)
├── sudo.rs           # Sudo credential management (Unix only)
└── engine/
    ├── mod.rs        # Module exports
    ├── types.rs      # Core types: Manager, Action, DetectedTool, ScanReport
    ├── scan.rs       # System scanning, tool detection
    ├── detect.rs     # Path-based manager detection (provenance)
    ├── filter.rs     # Action filtering (--only/--skip)
    └── managers/
        ├── mod.rs    # PackageManager trait, create_manager() factory
        └── *.rs      # Individual manager implementations
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

Create `src/engine/managers/mymanager.rs`:

```rust
use super::{Action, Manager, PackageManager};

pub struct MyManagerManager;

impl PackageManager for MyManagerManager {
    fn update_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::MyManager,
            "mymanager refresh",
            "Refresh MyManager index",
            false,  // Set true if needs sudo
        )]
    }

    fn upgrade_actions(&self) -> Vec<Action> {
        vec![Action::new(
            Manager::MyManager,
            "mymanager upgrade --all",
            "Upgrade MyManager packages",
            false,
        )]
    }
}
```

### Step 3: Register in mod.rs

In `src/engine/managers/mod.rs`:

```rust
// Add module declaration (with #[cfg] if platform-specific)
mod mymanager;
pub use mymanager::MyManagerManager;

// Add case in create_manager()
pub fn create_manager(manager: Manager) -> Option<Box<dyn PackageManager>> {
    match manager {
        // ... existing cases
        Manager::MyManager => Some(Box::new(MyManagerManager)),
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

Use conditional compilation for platform-specific managers:

```rust
#[cfg(target_os = "macos")]
mod brew;

#[cfg(target_os = "linux")]
mod apt;

#[cfg(target_os = "windows")]
mod winget;
```

Cross-platform managers (npm, cargo, asdf, etc.) have no `#[cfg]` attribute.

## Testing

### Unit Tests

```bash
cargo test
```

### Manual Testing

```bash
cargo run -- --dry-run      # Preview actions without executing
cargo run -- --status       # Check for outdated packages
cargo run -- --verbose      # See command output
cargo run -- --only brew    # Test specific manager
```

## Build & Release

### Local Build

```bash
cargo build --release
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
