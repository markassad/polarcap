# polarcap

A Polars interface to pcap (Packet Capture) data

[![CI](https://github.com/markassad/polarcap/workflows/CI/badge.svg)](https://github.com/markassad/polarcap/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![PyPI version](https://badge.fury.io/py/polarcap.svg)](https://badge.fury.io/py/polarcap)
[![codecov](https://codecov.io/gh/markassad/polarcap/branch/main/graph/badge.svg)](https://codecov.io/gh/markassad/polarcap)

## Overview

polarcap is a high-performance Python library for analyzing network packet capture (pcap) files using Polars DataFrames. It combines the speed of Rust for parsing with the powerful data analysis capabilities of Polars.

## Features

- **High Performance**: Core parsing implemented in Rust for maximum speed
- **Polars Integration**: Returns data as Polars DataFrames for efficient analysis
- **Easy to Use**: Pythonic API with familiar Polars patterns
- **Comprehensive**: Support for various pcap file formats

## Installation

```bash
pip install polarcap
```

## Quick Start

```python
import polarcap

# Coming soon: Read pcap file into a Polars DataFrame
# df = polarcap.read_pcap("capture.pcap")
# print(df.head())
```

## Development

### Prerequisites

- Rust (latest stable)
- Python 3.8+
- [uv](https://docs.astral.sh/uv/) (recommended) or pip

### Building from Source

#### Using uv (Recommended)

```bash
git clone https://github.com/markassad/polarcap
cd polarcap

# Install dependencies and set up development environment
make install  # This installs deps AND sets up pre-commit hooks

# Build the Rust extension
make build

# Or manually:
uv sync --group dev
uv run pre-commit install  # Set up git hooks
uv run maturin develop
```

#### Using pip

```bash
git clone https://github.com/markassad/polarcap
cd polarcap
pip install -e ".[dev]"
maturin develop
```

### Running Tests

#### Using uv

```bash
# Rust tests
cargo test                    # Run tests in current crate (with helpful workspace hint)
cargo test --workspace       # Run ALL tests in workspace (recommended)

# Python tests
uv run pytest

# Benchmarks
cargo bench --workspace

# Or use the Makefile:
make rust-test               # Run all workspace tests (cargo test --workspace)
make test                    # Run Python tests
make bench                   # Run benchmarks
```

#### Using pip

```bash
# Rust tests
cargo test                    # Run tests in current crate (with helpful workspace hint)
cargo test --workspace       # Run ALL tests in workspace (recommended)

# Python tests
pytest

# Benchmarks
cargo bench --workspace
```

### Code Quality

#### Using uv

```bash
# Format code
uv run ruff format python/
cargo fmt

# Lint code
uv run ruff check python/
cargo clippy

# Or use the Makefile:
make fmt   # Format all code
make lint  # Lint all code
```

#### Using pip

```bash
# Rust
cargo clippy
cargo fmt

# Python
pytest --cov
```

## Project Structure

```
polarcap/
├── src/              # Python bindings (PyO3)
├── polarcap-core/    # Core Rust library (pure Rust, testable with cargo test)
│   ├── src/          # Core parsing logic
│   └── benches/      # Rust benchmarks
├── python/polarcap/  # Python package with API
├── tests/            # Python integration tests
├── Cargo.toml        # Workspace configuration
└── pyproject.toml    # Python packaging configuration
```

The project uses a **workspace structure** to separate concerns:
- **`polarcap-core`**: Pure Rust library with all the core logic - fully testable with `cargo test`
- **`polarcap` (root)**: PyO3 Python bindings that expose the core functionality to Python

### Testing
- `cargo test` runs tests in the current crate (root) with a helpful hint
- `cargo test --workspace` runs ALL tests in the workspace (recommended)
- `make rust-test` automatically runs all workspace tests

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Workflow

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/yourusername/polarcap.git`
3. **Set up** the development environment: `make install` (installs deps + pre-commit hooks)
4. **Create** a feature branch: `git checkout -b my-feature`
5. **Make** your changes and add tests
6. **Test** your changes: `make test lint fmt`
7. **Commit** your changes (pre-commit hooks will run automatically)
8. **Push** to the branch: `git push origin my-feature`
9. **Submit** a pull request

### Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com/) to run checks before each commit:

- **Python**: `ruff` formatting and linting
- **Rust**: `cargo fmt` formatting checks
- **General**: file formatting, YAML/TOML validation, merge conflict detection
- **Tests**: runs Python tests when test files change

The hooks are automatically installed when you run `make install`. You can also:

```bash
# Run all hooks manually
make pre-commit-run

# Skip hooks for a commit (not recommended)
git commit --no-verify -m "message"

# Update hook versions
uv run pre-commit autoupdate
```

### Continuous Integration

This project uses GitHub Actions for CI/CD:

- **CI Pipeline** (`ci.yml`):
  - Tests Rust code with `cargo test`
  - Tests Python code across Python 3.8-3.12 on Ubuntu, macOS, and Windows
  - Runs linting and formatting checks
  - Runs security audits
  - Generates coverage reports

- **Release Pipeline** (`release.yml`):
  - Builds wheels for all platforms when you create a git tag
  - Creates GitHub releases automatically
  - Publishes to PyPI using trusted publishing

- **Dependency Updates**: Dependabot automatically creates PRs for dependency updates

### Making a Release

1. Update version in `Cargo.toml` and `pyproject.toml`
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.2.0`
4. Push the tag: `git push origin v0.2.0`
5. The release workflow will automatically build and publish

## License

This project is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
