# Contributing to distrovmm

Thank you for your interest in contributing to distrovmm! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/distrovmm.git
   cd distrovmm
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- libkrun and its dependencies (for full VM functionality)

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Style

We follow standard Rust conventions:

- Run `cargo fmt` before committing to format your code
- Run `cargo clippy` to catch common mistakes and improve code quality
- Add documentation comments for public APIs

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

## Project Structure

```
distrovmm/
├── src/
│   ├── main.rs       # Entry point
│   ├── cli.rs        # Command-line interface
│   ├── oci.rs        # OCI image pulling
│   ├── rootfs.rs     # Rootfs extraction
│   └── vm.rs         # VM management
├── tests/            # Integration tests
├── examples/         # Usage examples
└── Cargo.toml        # Dependencies and metadata
```

## Making Changes

### Types of Contributions

We welcome:

1. **Bug fixes** - Fix issues in existing code
2. **Features** - Implement new functionality described in the README roadmap
3. **Documentation** - Improve docs, add examples, fix typos
4. **Tests** - Add test coverage for existing code
5. **Performance** - Optimize slow code paths

### Commit Messages

Write clear commit messages:

- Use the imperative mood ("Add feature" not "Added feature")
- First line should be a short summary (50 chars or less)
- Add detailed explanation in the body if needed

Example:
```
Add support for private registries

- Implement authentication with custom registries
- Add --registry flag to specify custom registry URL
- Update documentation with examples
```

### Pull Request Process

1. **Update tests** - Add or update tests for your changes
2. **Update documentation** - Document new features or API changes
3. **Run tests locally** - Ensure all tests pass
4. **Format code** - Run `cargo fmt` and `cargo clippy`
5. **Create PR** - Submit your pull request with a clear description

### PR Description Template

```markdown
## Description
Brief description of what this PR does

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested your changes

## Checklist
- [ ] Code follows project style guidelines
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] All tests pass locally
```

## Priority Areas for Contribution

### 1. libkrun Integration (High Priority)

The most important missing piece is full libkrun integration:

- Create Rust bindings to libkrun (or use existing crate if available)
- Implement kernel extraction from rootfs
- Configure libkrun with appropriate parameters
- Set up console I/O for interactive use
- Test with multiple distributions

### 2. Distribution Support

Add support for more Linux distributions:

- Debian and derivatives
- Arch Linux
- Alpine Linux (currently partially implemented)
- OpenSUSE
- Distribution-specific kernel handling

### 3. Features from Roadmap

See README.md for the complete roadmap:

- Volume mounting (`-v` flag)
- Networking and port forwarding
- Image management commands (list, remove, etc.)
- Pre-compiled binaries for releases

### 4. Testing and Validation

- Add more comprehensive tests
- Test with various container images
- Performance benchmarking
- Security auditing

## Code Review Process

All submissions require review. We'll review your PR and:

- Check code quality and style
- Verify tests are adequate
- Ensure documentation is updated
- Test functionality if possible

Please be patient - maintainers review PRs as time permits.

## Getting Help

- **Issues**: Check existing issues or create a new one
- **Discussions**: Start a discussion for ideas or questions
- **Documentation**: See IMPLEMENTATION.md for technical details

## Code of Conduct

Be respectful and constructive:

- Use welcoming and inclusive language
- Respect differing viewpoints
- Accept constructive criticism gracefully
- Focus on what's best for the community

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0, the same license as the project.
