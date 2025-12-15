# polarcap Documentation

## Overview

polarcap is a Python library that provides a Polars interface to pcap (Packet Capture) data. It combines the high performance of Rust for parsing with the powerful data analysis capabilities of Polars.

## Architecture

- **Rust Core**: High-performance pcap parsing and data extraction
- **Python API**: Intuitive interface built on Polars DataFrames
- **PyO3 Bindings**: Seamless integration between Rust and Python

## Installation

```bash
pip install polarcap
```

For development:

```bash
git clone https://github.com/markassad/polarcap
cd polarcap
pip install -e ".[dev]"
maturin develop
```

## Usage

Coming soon...

## Development

### Building

```bash
# Build Rust code
cargo build --release

# Build and install Python package
maturin develop --release
```

### Testing

```bash
# Run Rust tests
cargo test

# Run Python tests
pytest

# Run benchmarks
cargo bench
```

### Linting

```bash
# Rust
cargo clippy
cargo fmt

# Python
pytest --cov
```

## API Reference

Coming soon...

## Contributing

Contributions are welcome! Please see the contributing guidelines.

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
