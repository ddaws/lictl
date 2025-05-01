#!/bin/bash
set -e

# Check if Rust is already installed
if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    echo "Rust is already installed. Updating..."
    rustup update
else
    echo "Installing Rust..."
    # Install Rust using rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Add Rust to the PATH for the current session
    source "$HOME/.cargo/env"
fi

# Verify installation
echo "Rust installation complete:"
rustc --version
cargo --version

echo "Rust setup completed successfully!"