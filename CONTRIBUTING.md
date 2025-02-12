# Contributing to Orbitus

First off, thank you for considering contributing to Orbitus! It's people like you that make Orbitus such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by the Orbitus Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* Use a clear and descriptive title
* Describe the exact steps which reproduce the problem
* Provide specific examples to demonstrate the steps
* Describe the behavior you observed after following the steps
* Explain which behavior you expected to see instead and why
* Include screenshots and animated GIFs if possible

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* Use a clear and descriptive title
* Provide a step-by-step description of the suggested enhancement
* Provide specific examples to demonstrate the steps
* Describe the current behavior and explain which behavior you expected to see instead
* Explain why this enhancement would be useful

### Pull Requests

* Fill in the required template
* Do not include issue numbers in the PR title
* Follow the Rust styleguides
* Include thoughtfully-worded, well-structured tests
* Document new code
* End all files with a newline

## Styleguides

### Git Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line

### Rust Styleguide

* Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/style-guide.html)
* Use `rustfmt` to format your code
* Run `clippy` and fix any warnings before submitting
* Write documentation for public APIs
* Include unit tests for new code

## Development Process

1. Fork the repo
2. Create a new branch from `main`
3. Make your changes
4. Run tests and ensure CI passes
5. Submit pull request

### Setting up development environment

```bash
# Clone your fork
git clone git@github.com:your-username/orbitus.git

# Add upstream remote
git remote add upstream https://github.com/original/orbitus.git

# Install dependencies
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with logging
RUST_LOG=debug cargo test
```

## Community

* Join our [Discord server](https://discord.gg/orbitus)
* Follow us on [Twitter](https://twitter.com/OrbitusNetwork)
* Read our [Blog](https://blog.orbitus.network)

## Questions?

Feel free to contact the project maintainers if you have any questions.
