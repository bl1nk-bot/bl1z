# Security Policy

## Supported Versions

We take security seriously. This section outlines our security policy and how to report security vulnerabilities.

### Version Support

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in bl1z, please help us by reporting it responsibly.

### How to Report

**DO NOT** create public GitHub issues for security vulnerabilities.

Instead, please report security vulnerabilities by emailing:
- **Email**: security@bl1z.dev
- **Subject**: `[SECURITY] Vulnerability Report`

### What to Include

Please include the following information in your report:

- **Description**: A clear description of the vulnerability
- **Impact**: Potential impact and severity
- **Steps to reproduce**: Detailed steps to reproduce the issue
- **Proof of concept**: Code or steps demonstrating the vulnerability
- **Environment**: Rust version, OS, dependencies used
- **Suggested fix**: If you have suggestions for fixing the issue

### Response Timeline

We will acknowledge your report within **48 hours** and provide a more detailed response within **7 days** indicating our next steps.

We will keep you informed about our progress throughout the process of fixing the vulnerability.

## Security Considerations

### Memory Safety

bl1z is written in Rust and benefits from Rust's memory safety guarantees:
- No buffer overflows
- No use-after-free errors
- No null pointer dereferences
- Thread safety (when used correctly)

### Input Validation

- All user inputs are validated
- Expression parsing includes bounds checking
- Type validation prevents invalid operations
- Error messages avoid leaking sensitive information

### Dependencies

- We regularly audit dependencies for known vulnerabilities
- Dependencies are kept up-to-date
- Only minimal, well-audited dependencies are used

### Best Practices

When using bl1z in production:

1. **Validate Inputs**: Always validate formula inputs before evaluation
2. **Limit Complexity**: Set reasonable limits on expression complexity
3. **Sandbox Execution**: Run in isolated environments if processing untrusted formulas
4. **Monitor Performance**: Watch for unusual performance patterns
5. **Keep Updated**: Apply security updates promptly

## Security Features

### Safe Evaluation

- **Type Safety**: Runtime type checking prevents invalid operations
- **Bounds Checking**: Array and string operations are bounds-checked
- **Recursion Limits**: Prevents infinite recursion attacks
- **Resource Limits**: Configurable limits on evaluation complexity

### Error Handling

- **No Information Leakage**: Error messages don't reveal internal state
- **Controlled Output**: Evaluation results are validated before return
- **Logging**: Security-relevant events can be logged

### Code Quality

- **No Unsafe Code**: Zero unsafe blocks in the codebase
- **Comprehensive Testing**: High test coverage including edge cases
- **Static Analysis**: Clippy and other tools catch potential issues
- **Code Review**: All changes undergo security review

## Third-Party Security

### Reporting Dependencies

We monitor and address security issues in our dependencies:

- **jiff**: Our date/time library - actively maintained with good security track record
- **Rust Ecosystem**: Benefits from Rust's security-focused design

### Supply Chain Security

- **Minimal Dependencies**: Only essential dependencies included
- **Pinned Versions**: Dependencies are pinned to specific versions
- **Regular Updates**: Dependencies updated regularly with security patches

## Incident Response

In the event of a confirmed security vulnerability:

1. **Immediate Assessment**: Evaluate impact and severity
2. **Fix Development**: Develop and test security fix
3. **Coordinated Release**: Release fix with vulnerability details
4. **Communication**: Notify users through appropriate channels

## Contact Information

For security-related questions or concerns:
- **Security Issues**: security@bl1z.dev
- **General Support**: support@bl1z.dev
- **GitHub Issues**: For non-security issues only

## Acknowledgments

We appreciate the security research community for helping keep open source software secure. Responsible disclosure is valued and recognized.
