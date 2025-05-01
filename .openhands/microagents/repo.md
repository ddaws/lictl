# Repository Guidelines for OpenHands

## Build Process

When working with this Rust project:

NEVER run `cargo check`, `cargo build`, or `cargo test`. Rely on Github workflows to build and test by pushing changes and WAITING FOR CI TO COMPLETE BEFORE ASSUMING THE TASK HAS BEEN COMPLETED SUCCESSFULLY. Check the results of the CI. If the CI failed look at the reason and attempt to fix the failure.

ALWAYS run `cargo fmt --all` before committing changes to ensure they pass formatting.

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
