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

# Install pre-commit
if command -v pre-commit &> /dev/null; then
    echo "pre-commit is already installed. Updating..."
    pip install --upgrade pre-commit
else
    echo "Installing pre-commit..."
    pip install pre-commit
fi

# Verify pre-commit installation
echo "pre-commit installation complete:"
pre-commit --version

# Install pre-commit hooks
echo "Installing pre-commit hooks..."
cd "$(git rev-parse --show-toplevel)" # Navigate to the root of the git repository
pre-commit install

echo "Setup completed successfully!"