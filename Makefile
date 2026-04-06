# Xorion Build Automation

## Help

```bash
make help
```

Show all available commands with descriptions.

## Build Commands

### Build Project
```bash
make build
```
Compile the entire workspace in release mode.

### Build Debug
```bash
make build-debug
```
Compile the entire workspace in debug mode (faster, for development).

### Build Specific Crate
```bash
make build-crate CRATE=xorion-sdk
```
Build a specific crate within the workspace.

## Test Commands

### Run All Tests
```bash
make test
```
Run all tests across the workspace with output capture disabled.

### Run Tests Verbose
```bash
make test-verbose
```
Run all tests with verbose output.

### Run Specific Test
```bash
make test-one TEST_NAME=test_wallet_creation
```
Run a specific test by name.

### Run Tests with Coverage
```bash
make coverage
```
Run tests and generate code coverage report (requires `cargo-tarpaulin`).

### Test Documentation
```bash
make test-doc
```
Test code examples in documentation.

## Run Commands

### Run GUI
```bash
make run-gui
```
Launch the desktop GUI application.

### Run Redox Scheme Daemon
```bash
make run-scheme
```
Start the Redox OS `wallet:/` scheme handler daemon.

### Run Demo Example
```bash
make run-demo
```
Execute the demo example showcasing wallet functionality.

### Run CLI
```bash
make run-cli
```
Launch the command-line interface.

## Code Quality

### Format Code
```bash
make fmt
```
Format all code using `rustfmt`.

### Check Formatting
```bash
make fmt-check
```
Check if code is properly formatted (without modifying).

### Run Clippy
```bash
make clippy
```
Run Rust linter with all warnings enabled.

### Run Clippy (Fix)
```bash
make clippy-fix
```
Run linter and automatically fix issues where possible.

### Check Documentation
```bash
make doc-check
```
Verify documentation builds without warnings.

### Generate Documentation
```bash
make docs
```
Generate HTML documentation in `target/doc/`.

## Cleanup

### Clean Build Artifacts
```bash
make clean
```
Remove all compiled artifacts.

### Deep Clean
```bash
make clean-all
```
Remove build artifacts, dependencies, and generated files.

### Clean Specific Crate
```bash
make clean-crate CRATE=xorion-sdk
```
Clean a specific crate's build artifacts.

## Dependencies

### Update Dependencies
```bash
make update-deps
```
Update all dependencies to latest compatible versions.

### Audit Dependencies
```bash
make audit
```
Check for security vulnerabilities in dependencies (requires `cargo-audit`).

### Install Dev Tools
```bash
make install-dev-tools
```
Install development tools: rustfmt, clippy, cargo-audit, cargo-tarpaulin.

## Docker

### Start Development Environment
```bash
make docker-up
```
Start Docker Compose development environment (IPFS node, etc.).

### Stop Development Environment
```bash
make docker-down
```
Stop and remove Docker containers.

### Rebuild Docker
```bash
make docker-build
```
Rebuild Docker images.

### View Docker Logs
```bash
make docker-logs
```
View logs from running containers.

## Release

### Create Release Build
```bash
make release
```
Create optimized release build.

### Build All Targets
```bash
make build-all-targets
```
Build for all supported platforms (Linux, macOS, Windows).

### Generate Changelog
```bash
make changelog
```
Generate changelog from git commits.

### Bump Version
```bash
make bump-version VERSION=1.0.0
```
Bump version number across all crates.

## CI/CD

### Run CI Checks
```bash
make ci
```
Run all checks performed in CI: build, test, clippy, fmt.

### Pre-commit Hook
```bash
make pre-commit
```
Run before committing: fmt, clippy, test.

## Benchmarking

### Run Benchmarks
```bash
make bench
```
Run performance benchmarks (requires nightly Rust).

### Profile Build
```bash
make profile
```
Build with profiling enabled for performance analysis.

## Miscellaneous

### Show Workspace Info
```bash
make info
```
Display workspace information and configuration.

### List Crates
```bash
make list-crates
```
List all crates in the workspace with versions.

### Check MSRV
```bash
make check-msrv
```
Verify Minimum Supported Rust Version compatibility.

### Generate README
```bash
make readme
```
Regenerate README.md from templates.

## Quick Reference

| Command | Description |
|---------|-------------|
| `make build` | Build project |
| `make test` | Run tests |
| `make run-gui` | Launch GUI |
| `make fmt` | Format code |
| `make clippy` | Run linter |
| `make clean` | Clean artifacts |
| `make docs` | Generate docs |
| `make docker-up` | Start dev environment |
| `make release` | Create release build |
| `make help` | Show this help |

## Environment Variables

- `RUST_LOG`: Set log level (default: `debug`)
- `NETWORK`: Target network (default: `devnet`)
- `CARGO_PROFILE_RELEASE_LTO`: Enable LTO for releases (default: `true`)

## Requirements

- Rust 1.75+ (stable)
- GNU Make
- Docker & Docker Compose (optional)
- IPFS node (provided via Docker)

## License

This Makefile is part of the Xorion project and licensed under the same terms.
