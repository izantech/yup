# Contributing to yup

Thank you for your interest in contributing to `yup`!

## Reporting Issues

### Bug Reports

Please open a GitHub Issue and include:
- Your operating system and version
- Output of `yup --version`
- Steps to reproduce the issue
- Expected vs actual behavior
- Relevant log output (run `yup log` to view logs)

### Feature Requests

Open a GitHub Issue with `[Feature]` in the title. Describe:
- The use case and problem you're solving
- Proposed behavior

For new package managers, include:
- Target platform (macOS, Linux, Windows, or cross-platform)
- Commands for update, upgrade, and status check
- How to detect if the manager is installed

## Development Setup

### Prerequisites

- Rust 1.92.0+ (Edition 2024)
- Target platform for testing
- rustup (recommended; honors `rust-toolchain.toml`)

### Building

```bash
git clone https://github.com/izantech/yup
cd yup
./dev build
./dev run -- --help
```

### Testing

```bash
./dev test
./dev run -- --dry-run  # Preview actions without executing
```

## Code Style

### Formatting

Before committing:
```bash
./dev check  # fmt --check + clippy + tests
```

### Conventions

- Use `anyhow::Result` for error handling
- Use `tracing` macros for logging (`debug!`, `info!`)
- Platform-specific code uses `#[cfg(target_os = "...")]`

## Pull Request Process

1. Fork the repository and create a feature branch
2. Make your changes
3. Run `cargo fmt && cargo clippy && cargo test`
4. Submit a PR against `main`
5. Use conventional commit prefixes: `feat:`, `fix:`, `docs:`, `refactor:`

## Adding a Package Manager

See [DEVELOPMENT.md](DEVELOPMENT.md) for a step-by-step guide.

## Releasing

1. Bump version in `Cargo.toml` and rename `[Unreleased]` in `CHANGELOG.md` to `[x.y.z] - date`
2. Commit: `chore: release yup version x.y.z`
3. Run `cargo build` to update `Cargo.lock`, commit: `chore: update Cargo.lock for vx.y.z`
4. Push to main
5. Create and push an annotated tag: `git tag -a vx.y.z -m "Release vx.y.z" && git push origin vx.y.z`

The GitHub Actions workflow then automatically:
- Builds binaries for all platforms (macOS, Linux, Windows x64/ARM64)
- Creates a GitHub Release with inline SHA256 checksums
- Publishes to crates.io
- Updates the Homebrew formula
- Submits to winget (Windows Package Manager)
- Publishes to Chocolatey

## License

MIT. Your contributions will be licensed under the same terms.
