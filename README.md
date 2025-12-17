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
uv sync --group dev

# Build the Rust extension
uv run maturin develop

# Or use the convenient Makefile:
make install  # Install dependencies
make build    # Build Rust extension
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
# Python tests
uv run pytest

# Python tests with coverage
uv run pytest --cov=polarcap --cov-report=html

# Rust tests
cargo test

# Benchmarks
cargo bench

# Or use the Makefile:
make test      # Run Python tests
make test-cov  # Run with coverage
make bench     # Run benchmarks
```

#### Using pip

```bash
# Rust tests
cargo test

# Python tests
pytest

# Benchmarks
cargo bench
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
├── src/              # Rust source code for parsing
├── python/polarcap/  # Python package with API
├── tests/            # Python and Rust tests
├── benches/          # Rust benchmarks
├── docs/             # Documentation
├── Cargo.toml        # Rust dependencies
└── pyproject.toml    # Python packaging configuration
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Workflow

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/yourusername/polarcap.git`
3. **Set up** the development environment: `make install`
4. **Create** a feature branch: `git checkout -b my-feature`
5. **Make** your changes and add tests
6. **Test** your changes: `make test lint fmt`
7. **Commit** your changes: `git commit -am 'Add some feature'`
8. **Push** to the branch: `git push origin my-feature`
9. **Submit** a pull request

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
