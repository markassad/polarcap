# Copilot Instructions for polarcap

## Project Overview

polarcap is a Python library that provides a Polars interface to pcap (Packet Capture) data. The project enables users to analyze network packet capture files using the powerful Polars data manipulation library.

## Development Guidelines

### Code Style and Standards

- Follow PEP 8 style guidelines for Python code
- Use type hints for function parameters and return values
- Write descriptive docstrings for all public functions, classes, and modules using Google-style docstrings
- Keep functions focused and single-purpose
- Use meaningful variable and function names that clearly convey intent

### Project Structure

- Keep the codebase modular and well-organized
- Separate concerns: parsing logic, data transformation, and API interfaces should be in separate modules
- Use clear directory structure (e.g., `src/polarcap/` for main code, `tests/` for tests, `docs/` for documentation)

### Dependencies

- Primary dependencies:
  - **Polars**: For high-performance data manipulation
  - **scapy** or **dpkt**: For pcap file parsing (to be determined)
- Keep dependencies minimal and well-justified
- Pin versions in requirements files for reproducibility

### Testing

- Write comprehensive unit tests for all new functionality
- Use pytest as the testing framework
- Aim for high test coverage (>80%)
- Include edge cases and error handling tests
- Test with various pcap file formats and sizes
- Add integration tests for end-to-end workflows

### Performance Considerations

- Optimize for large pcap files (streaming/chunking approach)
- Leverage Polars' lazy evaluation where applicable
- Profile code for memory usage with large datasets
- Document performance characteristics and limitations

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

- Design intuitive, Polars-like API for familiarity
- Support both eager and lazy evaluation patterns
- Provide sensible defaults while allowing customization
- Return Polars DataFrames/LazyFrames for consistency

### Git Workflow

- Write clear, descriptive commit messages
- Keep commits focused and atomic
- Use feature branches for development
- Update documentation alongside code changes

## Common Tasks

### Adding a New Feature

1. Write tests first (TDD approach)
2. Implement the feature with proper type hints and docstrings
3. Ensure all tests pass
4. Update relevant documentation
5. Consider performance implications

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
