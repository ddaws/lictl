# lictl - Lichess CLI Tool

A command-line interface for interacting with Lichess.org's API to make it easy to script your way around Lichess.

## Installation

```bash
cargo install lictl
```

## Authentication

Before using commands that require authentication, you'll need to log into Lichess:

```bash
lictl login

# Verify your authentication
lictl whoami
```

## Commands

### Broadcasts

```bash
# Get broadcast details by ID
lictl broadcasts get --by-id <broadcast-id>

# Get broadcast details by round ID
lictl broadcasts get --by-round <round-id>

# Export broadcast PGN
lictl broadcasts export <broadcast-id>
```

### Broadcast Rounds

```bash
# Get broadcast round details
lictl broadcast-rounds get <round-id>
```

### Studies

```bash
# Import PGN into a study
lictl studies import <study-id> "<pgn-content>"
```

### Generic API Requests

```bash
# Make GET requests to any Lichess API endpoint
lictl req get /account
lictl req get /broadcast page=2 nb=10
```

## Scripts

The project includes helpful scripts in the `scripts/` directory:

### copy-broadcast-to-study.sh

Copies a broadcast's games into a study:

```bash
./scripts/copy-broadcast-to-study.sh <round-id> <study-id>
```

## Development

```bash
# Run tests
cargo test

# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Pre-commit Hooks

This project uses pre-commit hooks to ensure code quality. The hooks run `cargo fmt` and `cargo clippy` before each commit.

To set up the pre-commit hooks:

```bash
# Run the setup script
./.openhands/setup.sh

# Or manually install pre-commit and the hooks
pip install pre-commit
pre-commit install
```

The hooks will automatically run when you commit changes. You can also run them manually:

```bash
# Run all hooks on all files
pre-commit run --all-files

# Run a specific hook
pre-commit run cargo-fmt
pre-commit run cargo-clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes (pre-commit hooks will ensure code quality)
4. Push to the branch
5. Create a Pull Request
