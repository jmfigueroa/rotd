# Contributing to ROTD

> Guidelines for contributing to the Runtime-Oriented Test Discipline project

## 🎯 Getting Started

1. **Fork the repository** and clone your fork
2. **Install dependencies**: Rust toolchain and `cargo`
3. **Build locally**: `cargo build`
4. **Run tests**: `cargo test`
5. **Initialize ROTD**: `cargo run -- init` to test CLI

## 🔧 Development Workflow

### Setting Up
```bash
git clone https://github.com/yourusername/rotd.git
cd rotd
cargo build
cargo test
```

### Making Changes
1. **Create a branch**: `git checkout -b feature/your-feature`
2. **Make changes** following the code style
3. **Add tests** for new functionality
4. **Run the full test suite**: `cargo test`
5. **Check formatting**: `cargo fmt`
6. **Run clippy**: `cargo clippy`

### Testing Your Changes
```bash
# Test CLI functionality
cargo run -- init --force
cargo run -- check
cargo run -- agent info

# Test with real project
cd /path/to/test/project
/path/to/rotd/target/debug/rotd init
```

## 📋 Code Guidelines

### Rust Code Style
- **Use `cargo fmt`** for consistent formatting
- **Follow clippy suggestions**: `cargo clippy`
- **Add documentation** for public functions
- **Include tests** for new functionality
- **Handle errors properly**: Use `anyhow::Result` for error handling

### CLI Design Principles
- **Human-friendly by default**: Colored output, helpful messages
- **Agent mode available**: `--agent` flag for programmatic use
- **Validate input**: Schema validation for JSON inputs
- **Provide feedback**: Clear success/error messages
- **Support dry-run**: `--dry-run` for safety

### Adding New Commands
1. **Add to CLI enum** in `main.rs`
2. **Implement in appropriate module**: `human.rs` or `agent.rs`
3. **Add validation** and error handling
4. **Include tests** for the new functionality
5. **Update documentation**

### Schema Changes
- **Update JSON schemas** in `schema/` directory
- **Provide examples** in `examples/` directory
- **Update validation** in Rust code
- **Test with real data** to ensure compatibility

## 🧪 Testing

### Running Tests
```bash
cargo test                    # All tests
cargo test --lib             # Library tests only
cargo test integration       # Integration tests
```

### Test Structure
- **Unit tests**: In the same file as the code (`#[cfg(test)]`)
- **Integration tests**: In `tests/` directory
- **CLI tests**: Use `assert_cmd` for command-line testing

### Test Data
- **Use examples**: Reference files in `examples/` directory
- **Mock external dependencies**: No network calls in tests
- **Test error cases**: Ensure proper error handling

## 📚 Documentation

### Required Documentation
- **README updates**: For new features or major changes
- **CLI help text**: Keep `--help` output current
- **Code comments**: For complex logic or algorithms
- **Examples**: Update `examples/` directory as needed

### Documentation Style
- **Clear and concise**: Explain the "why" not just the "what"
- **Include examples**: Show actual usage patterns
- **Keep updated**: Documentation should match current behavior

## 🚀 Submitting Changes

### Pull Request Process
1. **Ensure tests pass**: `cargo test`
2. **Update documentation**: README, help text, examples
3. **Write clear commit messages**: Follow conventional commits
4. **Create pull request** with description of changes
5. **Respond to feedback**: Address review comments

### Commit Message Format
```
type(scope): description

body (optional)

footer (optional)
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Examples**:
- `feat(cli): add agent info command`
- `fix(validation): handle empty task IDs`
- `docs(readme): update installation instructions`

### PR Checklist
- [ ] Tests pass locally
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] Examples updated if needed
- [ ] No breaking changes (or documented)

## 🐛 Reporting Issues

### Bug Reports
Include:
- **ROTD version**: `rotd --version`
- **Operating system**: Linux/macOS/Windows
- **Steps to reproduce**: Minimal example
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Error messages**: Full error output

### Feature Requests
Include:
- **Use case**: Why is this needed?
- **Proposed solution**: How should it work?
- **Alternatives considered**: Other approaches
- **CLI impact**: How would it affect existing commands?

## 🔧 Development Environment

### Required Tools
- **Rust**: Latest stable version
- **Git**: For version control
- **Text editor**: VS Code with rust-analyzer recommended

### Recommended Setup
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install useful tools
cargo install cargo-watch    # Auto-rebuild on changes
cargo install cargo-audit    # Security auditing
cargo install cargo-edit     # Add/remove dependencies

# VS Code extensions
# - rust-analyzer
# - Error Lens
# - GitLens
```

### Development Commands
```bash
cargo watch -x build         # Auto-rebuild on changes
cargo watch -x test          # Auto-test on changes
cargo doc --open             # Generate and view docs
cargo audit                  # Check for security issues
```

## 🎨 Project Structure

```
src/
├── main.rs          # CLI entry point
├── agent.rs         # Agent mode commands
├── human.rs         # Human mode commands
├── schema.rs        # Data structures
├── fs_ops.rs        # File operations
├── pss.rs           # Progress scoring
├── audit.rs         # Audit logging
└── common.rs        # Shared utilities

examples/            # Example ROTD artifacts
schema/              # JSON schemas
.github/workflows/   # CI/CD configuration
```

## ❓ Getting Help

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Report bugs and request features
- **Documentation**: Check README and docs first
- **Code**: Look at examples in the repository

## 🤝 Code of Conduct

- **Be respectful**: Treat everyone with respect
- **Be constructive**: Provide helpful feedback
- **Be patient**: Not everyone has the same experience level
- **Be collaborative**: Work together toward common goals

Thank you for contributing to ROTD!