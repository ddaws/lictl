# Lichess CLI Tool (lictl) Proposal

## Core Design

lictl would follow these principles:

- Consistent command structure
- JSON output by default
- Authentication via API token
- Support for common operations on broadcasts and studies

## Command Structure

```
lictl [global-options] <resource> <command> [command-options]
```

### Global Options

```
--token <token>         Lichess API token (can also use LICHESS_TOKEN env var)
--output, -o <format>   Output format: json (default), yaml, table
--verbose, -v           Enable verbose output
--help, -h              Show help
--version               Show version
```

## Proposed Commands

### Authentication

```
lictl login --token <token>   # Store token for future use
lictl logout                  # Remove stored token
lictl whoami                  # Show current user info
```

### Broadcasts

```
# List broadcasts
lictl broadcasts list [--status <status>] [--limit <n>]

# Get broadcast details
lictl broadcasts get <broadcast-id>

# Create a new broadcast
lictl broadcasts create --name <name> [--description <desc>] [--official <true|false>]

# Update broadcast
lictl broadcasts update <broadcast-id> [--name <name>] [--description <desc>]

# Delete broadcast
lictl broadcasts delete <broadcast-id>

# Broadcast rounds
lictl broadcasts rounds list <broadcast-id>
lictl broadcasts rounds get <broadcast-id> <round-id>
lictl broadcasts rounds create <broadcast-id> --name <name> [--description <desc>]
lictl broadcasts rounds update <broadcast-id> <round-id> [--name <name>] [--description <desc>]
lictl broadcasts rounds delete <broadcast-id> <round-id>

# Push PGN to a broadcast round
lictl broadcasts rounds push <broadcast-id> <round-id> --pgn <pgn-file>

# Start/stop broadcast
lictl broadcasts start <broadcast-id>
lictl broadcasts stop <broadcast-id>
```

### Studies

```
# List studies
lictl studies list [--limit <n>] [--mine] [--member <username>] [--public]

# Get study details
lictl studies get <study-id>

# Create a new study
lictl studies create --name <name> [--visibility <public|private>] [--description <desc>]

# Update study
lictl studies update <study-id> [--name <name>] [--visibility <public|private>] [--description <desc>]

# Delete study
lictl studies delete <study-id>

# Study chapters
lictl studies chapters list <study-id>
lictl studies chapters get <study-id> <chapter-id>
lictl studies chapters create <study-id> --name <name> [--pgn <pgn-file>]
lictl studies chapters update <study-id> <chapter-id> [--name <name>] [--pgn <pgn-file>]
lictl studies chapters delete <study-id> <chapter-id>

# Import/export PGN
lictl studies export <study-id> [--chapter <chapter-id>] [--output <file>]
lictl studies import <study-id> --pgn <pgn-file> [--name <chapter-name>]

# Study permissions
lictl studies permissions list <study-id>
lictl studies permissions add <study-id> --user <username> --role <contributor|member|viewer>
lictl studies permissions remove <study-id> --user <username>
```

### Utilities

```
# Search for broadcasts/studies
lictl search broadcasts <query> [--limit <n>]
lictl search studies <query> [--limit <n>]

# Export data
lictl export broadcast <broadcast-id> [--format pgn|json] [--output <file>]
lictl export study <study-id> [--format pgn|json] [--output <file>]
```

## Example Usage

```bash
# List your broadcasts
lictl broadcasts list --mine

# Create a new broadcast
lictl broadcasts create --name "Chess Championship 2023" --description "Live coverage"

# Create a round in the broadcast
lictl broadcasts rounds create abc123 --name "Round 1"

# Push PGN data to the round
lictl broadcasts rounds push abc123 xyz456 --pgn games.pgn

# List all public studies
lictl studies list --public --limit 10

# Create a new study
lictl studies create --name "My Analysis" --visibility public

# Add a chapter with PGN
lictl studies chapters create def456 --name "Game 1" --pgn game1.pgn

# Export a study as PGN
lictl studies export def456 --output my-study.pgn
```

This design provides a comprehensive CLI for working with Lichess broadcasts and studies, following the kubectl pattern of resource-based commands with consistent syntax. The tool outputs JSON by default but allows for other formats, and it supports authentication via API tokens.


