# Copilot Instructions for polarcap

## Project Overview

polarcap is a Python library that provides a Polars interface to pcap (Packet Capture) data. The project enables users to analyze network packet capture files using the powerful Polars data manipulation library.

**Architecture**: This is a hybrid Rust/Python project:
- **Rust**: Core parsing and extraction logic for high performance
- **Python**: User-facing API and Polars integration
- Rust code is exposed to Python via PyO3 bindings

## Development Guidelines

### Code Style and Standards

**Python Code:**
- Follow PEP 8 style guidelines
- Use type hints for function parameters and return values
- Write descriptive docstrings for all public functions, classes, and modules using Google-style docstrings
- Keep functions focused and single-purpose
- Use meaningful variable and function names that clearly convey intent

**Rust Code:**
- Follow Rust standard formatting (use `rustfmt`)
- Use `clippy` for linting and best practices
- Write comprehensive documentation comments (`///`) for public APIs
- Leverage Rust's type system and error handling (`Result<T, E>`)
- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)

### Project Structure

- Keep the codebase modular and well-organized
- Separate concerns: parsing logic (Rust), data transformation (Rust/Python), and API interfaces (Python)
- Expected directory structure:
  - `src/` - Rust source code for parsing and extraction
  - `python/polarcap/` - Python package with API and Polars integration
  - `tests/` - Python and Rust tests
  - `benches/` - Rust benchmarks for performance testing
  - `docs/` - Documentation

### Dependencies

**Rust Dependencies:**
- **polars** (Rust crate): Core data structures and operations
- **pcap-parser** or **pnet**: For pcap file parsing
- **pyo3**: Python bindings for Rust code
- **maturin** or **setuptools-rust**: Build tooling for Python packaging

**Python Dependencies:**
- **polars** (Python): High-performance data manipulation, exposed to users
- Keep Python dependencies minimal since heavy lifting is in Rust

**General:**
- Keep dependencies minimal and well-justified
- Pin versions in `Cargo.toml`, `pyproject.toml`, and requirements files for reproducibility

### Testing

**Rust Tests:**
- Write unit tests for Rust modules using `#[cfg(test)]`
- Use `cargo test` to run Rust tests
- Test pcap parsing logic, data extraction, and edge cases at the Rust level
- Add benchmarks in `benches/` to track performance

**Python Tests:**
- Use pytest as the testing framework for Python API tests
- Test the Python API and integration with Polars
- Add integration tests for end-to-end workflows

**General:**
- Aim for high test coverage (>80%) across both Rust and Python
- Test with various pcap file formats and sizes
- Include edge cases and error handling tests

### Performance Considerations

- **Rust is key for performance**: Implement parsing and extraction in Rust for speed
- Optimize for large pcap files (streaming/chunking approach in Rust)
- Leverage Polars' lazy evaluation in both Rust and Python where applicable
- Use zero-copy operations where possible when passing data between Rust and Python
- Profile Rust code with `cargo bench` and Python code with standard profilers
- Document performance characteristics and limitations
- Minimize Python overhead by doing heavy computation in Rust

### Error Handling

- Use descriptive exception messages
- Create custom exception classes for library-specific errors
- Handle malformed pcap files gracefully
- Validate input parameters early

### Documentation

- Maintain up-to-date README with installation, usage examples, and API reference
- Include docstrings with examples for complex functions
- Add type hints to improve IDE support and documentation generation
- Document performance characteristics and best practices

### API Design

- Design intuitive, Polars-like API for familiarity (Python layer)
- Support both eager and lazy evaluation patterns
- Provide sensible defaults while allowing customization
- Return Polars DataFrames/LazyFrames for consistency
- Keep the Python API simple and Pythonic; hide Rust complexity
- Expose Rust functionality through clean PyO3 bindings

### Build and Development

**Rust Development:**
- Use `cargo build` for building Rust code
- Use `cargo test` for running Rust tests
- Use `cargo clippy` for linting
- Use `cargo fmt` for formatting
- Use `cargo bench` for benchmarking

**Python Development:**
- Use `maturin develop` or similar to build and install the package locally with Rust extensions
- Use `pytest` for Python tests
- Ensure Rust components are built before running Python tests

**Development Workflow:**
- Test both Rust and Python layers when making changes
- Rebuild Rust extensions when modifying Rust code (`maturin develop`)

### Git Workflow

- Write clear, descriptive commit messages
- Keep commits focused and atomic
- Use feature branches for development
- Update documentation alongside code changes

## Common Tasks

### Adding a New Feature

1. Determine if the feature belongs in Rust (performance-critical) or Python (user-facing API)
2. Write tests first (TDD approach) - Rust tests for Rust code, pytest for Python
3. Implement the feature:
   - **Rust**: Add to appropriate module, expose via PyO3 if needed
   - **Python**: Add with proper type hints and docstrings
4. If adding Rust code exposed to Python, update PyO3 bindings
5. Ensure all tests pass (both `cargo test` and `pytest`)
6. Update relevant documentation
7. Consider performance implications and benchmark if needed

### Bug Fixes

1. Add a test that reproduces the bug
2. Fix the bug
3. Verify the test now passes
4. Check for similar issues elsewhere in the codebase

### Refactoring

1. Ensure comprehensive test coverage exists
2. Make incremental changes
3. Run tests frequently
4. Maintain backward compatibility where possible

## Security Considerations

- Validate all input from pcap files (malformed data, malicious content)
- Be cautious with file paths and prevent path traversal
- Document security assumptions and limitations
- Handle large files safely to prevent DoS scenarios
