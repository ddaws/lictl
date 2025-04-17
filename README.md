# lictl - Lichess CLI Tool

A command-line interface for interacting with Lichess.org's API to make it easy to script your way around Lichess.

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/lictl.git
cd lictl

# Build and install
cargo install --path crates/lictl
```

## Authentication

Before using commands that require authentication, you'll need to log in with a Lichess API token:

```bash
# Store your token securely
lictl login

# Verify your authentication
lictl whoami

# Remove stored token
lictl logout
```

You can generate a personal access token at: https://lichess.org/account/oauth/token

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

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

TODO
