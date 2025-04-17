#!/usr/bin/env bash

set -euo pipefail

if [ $# -ne 2 ]; then
    echo "Usage: $0 <broadcast-round-id> <study-id>"
    echo "Example: $0 abcd1234 efgh5678"
    exit 1
fi

ROUND_ID="$1"
STUDY_ID="$2"

# Get the broadcast info and extract the tour ID (which is the broadcast ID)
BROADCAST_JSON=$(lictl broadcasts get --by-round "$ROUND_ID")
BROADCAST_ID=$(echo "$BROADCAST_JSON" | jq -r '.tour.id')

if [ -z "$BROADCAST_ID" ]; then
    echo "Error: Could not find broadcast ID in response"
    exit 1
fi

# Get the PGN using the broadcast ID
PGN=$(lictl broadcasts export "$BROADCAST_ID")

# Import the PGN into the study
lictl studies import "$STUDY_ID" --pgn "$PGN"
