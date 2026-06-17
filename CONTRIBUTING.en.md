# Contributing to bl1z

Thank you for your interest in contributing to the bl1z! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

This project follows a code of conduct to ensure a welcoming environment for all contributors. By participating, you agree to:

- Be respectful and inclusive
- Focus on constructive feedback
- Accept responsibility for mistakes
- Show empathy towards other contributors
- Help create a positive community

## Getting Started

### Prerequisites

- **Rust**: Version 1.70 or higher (check with `rustc --version`)
- **Cargo**: Comes with Rust (check with `cargo --version`)
- **Git**: For version control

### Quick Setup

1. **Fork and Clone** the repository:
   ```bash
   git clone https://github.com/billlzzz26/bl1z.git
   cd bl1z
   ```

2. **Set up development environment**:
   ```bash
   cargo check  # Verify everything compiles
   cargo test   # Run the test suite
   ```

## Development Setup

### Installing Dependencies

For Ubuntu/Debian:
```bash
sudo apt update
sudo apt install -y build-essential
```

For macOS:
```bash
xcode-select --install
```

For Windows: Install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/) or use WSL.

### Development Tools

```bash
# Install additional tools
cargo install cargo-watch    # For file watching
cargo install cargo-edit     # For dependency management
cargo install cargo-audit    # For security auditing
```

### IDE Setup

**VS Code** (Recommended):
- Install Rust Analyzer extension
- Install "CodeLLDB" for debugging
- Install "crates" for dependency management

**Other Editors**:
- Any editor with Rust language support
- Rust Analyzer LSP provides excellent IDE features

## Development Workflow

### 1. Choose an Issue

- Check [GitHub Issues](https://github.com/bl1nk-bot/bl1z/issues) for open tasks
- Look for issues labeled `good first issue` or `help wanted`
- Comment on the issue to indicate you're working on it

### 2. Create a Branch

```bash
git checkout -b feature/my-feature
# or
git checkout -b fix/issue-number-description
```

### 3. Make Changes

- Write clear, focused commits
- Follow the coding standards below
- Add tests for new functionality
- Update documentation as needed

### 4. Run Checks

```bash
cargo check                    # Type checking
cargo fmt --all -- --check     # Code formatting
cargo clippy                   # Linting
cargo test --verbose           # Testing
cargo bench                    # Benchmarks (if applicable)
```

### 5. Submit Pull Request

- Push your branch to GitHub
- Create a Pull Request with a clear description
- Reference any related issues
- Request review from maintainers

## Coding Standards

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Use `rustfmt` for automatic formatting
- Use `clippy` for linting (all warnings must pass)

### Code Organization

- **Modules**: Logical separation of functionality
- **Functions**: Small, focused, single responsibility
- **Types**: Clear naming, comprehensive documentation
- **Error Handling**: Use `Result` types, meaningful error messages

### Documentation

- **Public APIs**: Must have comprehensive documentation
- **Complex logic**: Include explanatory comments
- **Examples**: Provide code examples where helpful

### Naming Conventions

- **Functions**: `snake_case`
- **Types/Enums**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

### Example Code Structure

```rust
/// Brief description of what this function does.
///
/// # Arguments
/// * `param` - Description of parameter
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// use bl1z::example_function;
/// let result = example_function(42);
/// assert_eq!(result, 84);
/// ```
pub fn example_function(param: i32) -> i32 {
    param * 2
}
```

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# Tests with output
cargo test -- --nocapture

# Doc tests
cargo test --doc
```

### Writing Tests

- **Unit Tests**: Test individual functions/components
- **Integration Tests**: Test larger functionality
- **Doc Tests**: Examples in documentation
- **Property Tests**: Use libraries like `proptest` for complex scenarios

### Test Coverage

- Aim for high test coverage (>80%)
- Test both success and error cases
- Include edge cases and boundary conditions

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = 42;
        let expected = 84;

        // Act
        let result = example_function(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_error_cases() {
        // Test error handling
        let result = example_function(-1);
        assert!(result.is_err());
    }
}
```

## Submitting Changes

### Commit Messages

Follow conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Testing
- `chore`: Maintenance

Examples:
```
feat(parser): add support for array literals
fix(eval): correct division by zero handling
docs(api): update function documentation
```

### Pull Request Guidelines

- **Title**: Clear, descriptive title
- **Description**: Explain what and why
- **Tests**: Include tests for new functionality
- **Documentation**: Update docs if needed
- **Breaking Changes**: Clearly mark and explain
- **Related Issues**: Reference issue numbers

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All existing tests pass
- [ ] New tests added
- [ ] Manual testing performed

## Checklist
- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] Documentation updated
- [ ] Commit messages follow conventions
```

## Reporting Issues

### Bug Reports

Please include:
- **Description**: Clear description of the issue
- **Steps to reproduce**: Minimal code example
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Environment**: Rust version, OS, etc.
- **Additional context**: Screenshots, logs, etc.

### Feature Requests

Please include:
- **Description**: What feature you'd like
- **Use case**: Why it's needed
- **Proposed solution**: How it could work
- **Alternatives**: Other approaches considered

### Security Issues

For security vulnerabilities:
- **DO NOT** create public GitHub issues
- Email maintainers directly
- See [Security Policy](SECURITY.md) for details

## Recognition

Contributors will be:
- Listed in CHANGELOG.md for significant changes
- Acknowledged in release notes
- Added to contributors list (if desired)

Thank you for contributing to bl1z! 🎉
