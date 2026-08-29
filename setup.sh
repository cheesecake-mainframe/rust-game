#!/usr/bin/env bash
#
# setup.sh — rust-game setup
#
# Checks prerequisites (git, rustc >= 1.88.0, cargo), clones external
# reference material into deps/, and builds the project.
# Safe to run multiple times.

set -euo pipefail

DEPS_DIR="deps"
RBE_REPO="https://github.com/rust-lang/rust-by-example.git"
RBE_DIR="$DEPS_DIR/rust-by-example"
RUSTLINGS_REPO="https://github.com/rust-lang/rustlings.git"
RUSTLINGS_DIR="$DEPS_DIR/rustlings"

# --- Check prerequisites ---

echo "Checking prerequisites..."

if ! command -v git &> /dev/null; then
    echo "Error: git is not installed."
    echo "Install git from https://git-scm.com/downloads and try again."
    exit 1
fi

if ! command -v rustc &> /dev/null; then
    echo "Error: rustc is not installed."
    echo "Install Rust from https://rustup.rs/ and try again."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed."
    echo "Install Rust from https://rustup.rs/ and try again."
    exit 1
fi

RUST_VERSION=$(rustc --version)
echo "  git:   $(git --version)"
echo "  rustc: $RUST_VERSION"
echo "  cargo: $(cargo --version)"

# Check minimum Rust version (1.88.0)
MIN_VERSION="1.88.0"
CURRENT_VERSION=$(rustc --version | awk '{print $2}')
if [ "$(printf '%s\n' "$MIN_VERSION" "$CURRENT_VERSION" | sort -V | head -n1)" != "$MIN_VERSION" ]; then
    echo "Error: Rust $MIN_VERSION or later is required. You have $CURRENT_VERSION."
    echo "Run: rustup update"
    exit 1
fi

echo "  All prerequisites met."
echo ""

# --- Create deps directory ---

mkdir -p "$DEPS_DIR"

# --- Clone Rust by Example ---

if [ -d "$RBE_DIR" ]; then
    echo "Rust by Example already exists at $RBE_DIR — skipping."
else
    echo "Cloning Rust by Example (reference material)..."
    if git clone --depth 1 "$RBE_REPO" "$RBE_DIR"; then
        echo "  Cloned successfully."
    else
        echo "  Warning: Failed to clone Rust by Example. This is optional — continuing."
    fi
fi

# --- Optionally clone Rustlings ---

echo ""
read -p "Clone Rustlings for reference? (optional) [y/N] " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ -d "$RUSTLINGS_DIR" ]; then
        echo "Rustlings already exists at $RUSTLINGS_DIR — skipping."
    else
        echo "Cloning Rustlings..."
        if git clone --depth 1 "$RUSTLINGS_REPO" "$RUSTLINGS_DIR"; then
            echo "  Cloned successfully."
        else
            echo "  Warning: Failed to clone Rustlings. This is optional — continuing."
        fi
    fi
fi

# --- Build rust-game ---

echo ""
echo "Building rust-game (first build takes 2-5 minutes, subsequent builds are fast)..."
cargo build --release
echo "Build complete."

# --- Done ---

echo ""
echo "Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Run: cargo run             (launches the TUI dashboard)"
echo "  2. Or:  cargo run -- watch    (start with the first exercise)"
echo "  3. See: cargo run -- --help   (for all commands)"
