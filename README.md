# polarcap

A Polars interface to pcap (Packet Capture) data

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

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

## License

This project is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
