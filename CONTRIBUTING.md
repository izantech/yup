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

- Rust 1.70+ (Edition 2021)
- Target platform for testing

### Building

```bash
git clone https://github.com/izantech/yup
cd yup
cargo build
cargo run -- --help
```

### Testing

```bash
cargo test
cargo run -- --dry-run  # Preview actions without executing
```

## Code Style

### Formatting

Before committing:
```bash
cargo fmt
cargo clippy
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

## License

MIT. Your contributions will be licensed under the same terms.
