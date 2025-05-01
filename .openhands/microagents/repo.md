# Repository Guidelines for OpenHands

## Build Process

When working with this Rust project:

- Always run `cargo check` before `cargo build` to save time checking for errors
- Always run `cargo fmt --check` to check for formatting errors before opening a PR

## Commit Messages

This repository follows the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification for commit messages.

### Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `build`: Changes that affect the build system or external dependencies
- `ci`: Changes to our CI configuration files and scripts
- `chore`: Other changes that don't modify src or test files

### Examples

```
feat: add new CLI command for listing resources
fix(parser): handle edge case in JSON parsing
docs: update installation instructions
```