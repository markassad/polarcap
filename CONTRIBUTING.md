# Contributing to polarcap

Thank you for your interest in contributing to polarcap! This document provides guidelines and instructions for contributing to the project.

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Python 3.8 or later
- maturin (for building the Python package)

### Setting up the Development Environment

```bash
# Clone the repository
git clone https://github.com/markassad/polarcap
cd polarcap

# Install Python dependencies
pip install -e ".[dev]"

# Build the Rust extension
maturin develop
```

## Code Style

### Rust

- Follow Rust standard formatting (use `rustfmt`)
- Use `clippy` for linting and best practices
- Write comprehensive documentation comments (`///`) for public APIs
- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)

### Python

- Follow PEP 8 style guidelines
- Use type hints for function parameters and return values
- Write descriptive docstrings using Google-style format
- Keep functions focused and single-purpose

## Testing

### Running Tests

```bash
# Rust tests
cargo test

# Python tests
pytest

# Benchmarks
cargo bench
```

### Writing Tests

- Write unit tests for Rust modules using `#[cfg(test)]`
- Use pytest for Python API tests
- Aim for high test coverage (>80%)
- Test edge cases and error handling

## Code Quality

```bash
# Rust
cargo fmt        # Format code
cargo clippy     # Run linter

# Python
pytest --cov    # Run tests with coverage
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure all tests pass
6. Run code formatting and linting
7. Commit your changes with clear, descriptive commit messages
8. Push to your fork
9. Open a Pull Request

## Pull Request Guidelines

- Provide a clear description of the changes
- Reference any related issues
- Ensure all tests pass
- Update documentation as needed
- Keep changes focused and atomic

## Questions?

Feel free to open an issue if you have questions or need help!
