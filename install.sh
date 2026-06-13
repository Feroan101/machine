#!/bin/sh

set -e

# Configuration
REPO="Feroan101/machine"
BINARY_NAME="machine"
INSTALL_DIR="/usr/local/bin"

# Help message
show_help() {
    echo "Machine Installer"
    echo "Usage: ./install.sh [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help    Show this help message"
    echo "  -p, --path    Installation directory (default: /usr/local/bin)"
}

# Parse arguments
while [ "$#" -gt 0 ]; do
    case "$1" in
        -h|--help) show_help; exit 0 ;;
        -p|--path) INSTALL_DIR="$2"; shift 2 ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# Detect Architecture
ARCH=$(uname -m)
OS=$(uname -s)

if [ "$OS" != "Linux" ]; then
    echo "Error: Machine is currently only supported on Linux."
    exit 1
fi

case "$ARCH" in
    x86_64) TARGET="x86_64-unknown-linux-musl" ;;
    aarch64) TARGET="aarch64-unknown-linux-musl" ;;
    *) echo "Error: Unsupported architecture $ARCH"; exit 1 ;;
esac

echo "Detected architecture: $ARCH ($TARGET)"

# Check for local build first
if [ -f "./target/release/machine" ]; then
    echo "Found local release build. Installing from source..."
    BINARY_SOURCE="./target/release/machine"
elif [ -f "./target/debug/machine" ]; then
    echo "Found local debug build. Installing from source..."
    BINARY_SOURCE="./target/debug/machine"
else
    # Fetch latest release
    echo "Fetching latest release version from GitHub..."
    LATEST_RELEASE=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")')

    if [ -z "$LATEST_RELEASE" ]; then
        echo "Error: Could not retrieve latest release version."
        exit 1
    fi

    echo "Latest release: $LATEST_RELEASE"

    # Download URL
    URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/machine-$TARGET.tar.gz"

    echo "Downloading $URL..."
    TEMP_DIR=$(mktemp -d)
    curl -fsSL "$URL" -o "$TEMP_DIR/machine.tar.gz"

    # Extract
    echo "Extracting..."
    tar -xzf "$TEMP_DIR/machine.tar.gz" -C "$TEMP_DIR"
    BINARY_SOURCE="$TEMP_DIR/$BINARY_NAME"
fi

echo "Installing to $INSTALL_DIR..."
if [ ! -w "$INSTALL_DIR" ]; then
    echo "Requesting sudo for installation to $INSTALL_DIR"
    sudo cp "$BINARY_SOURCE" "$INSTALL_DIR/$BINARY_NAME"
    sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
else
    cp "$BINARY_SOURCE" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
fi

# Cleanup
rm -rf "$TEMP_DIR"

echo "Success! Machine has been installed to $INSTALL_DIR."
echo "Run 'machine status' to get started."
