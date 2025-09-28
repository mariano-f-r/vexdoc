# Contributing to VexDoc

Thank you for your interest in contributing to VexDoc! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/vexdoc.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run clippy: `cargo clippy`
7. Format code: `cargo fmt`
8. Commit your changes: `git commit -m "Add your feature"`
9. Push to your fork: `git push origin feature/your-feature-name`
10. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+ (stable)
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Code Style

- Use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Follow Rust naming conventions
- Add documentation for public APIs
- Write tests for new functionality

## Testing

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_tests

# Specific test
cargo test test_name
```

### Test Examples

```bash
# Test with example files
cd examples/test_files
./test.sh
```

## Performance

VexDoc is optimized for speed and small binary size. When making changes:

- Consider performance implications
- Run benchmarks to measure impact
- Avoid unnecessary allocations in hot paths
- Use `cargo bench` to measure performance

## Documentation

- Update README.md for user-facing changes
- Add doc comments for new public APIs
- Update examples if needed
- Keep documentation clear and concise

## Pull Request Guidelines

- Keep PRs focused and atomic
- Include tests for new features
- Update documentation as needed
- Ensure all tests pass
- Follow the existing code style
- Write clear commit messages

## Issues

When reporting issues:

- Use the issue templates
- Provide clear reproduction steps
- Include system information
- Check existing issues first

## Questions?

Feel free to open an issue for questions or discussions!
