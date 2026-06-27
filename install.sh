#!/usr/bin/env bash
set -euo pipefail

REPO="pyrofast/mcplink"

echo "==> mcplink installer"
echo ""

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)  TARGET="unknown-linux-gnu" ;;
  darwin) TARGET="apple-darwin" ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64) TARGET_ARCH="x86_64" ;;
  aarch64|arm64) TARGET_ARCH="aarch64" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

BINARY="mcplink-${TARGET_ARCH}-${TARGET}"

# Fetch latest release tag
echo "==> Fetching latest release..."
TAG=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | cut -d'"' -f4)
if [ -z "$TAG" ]; then
  echo "Failed to fetch latest release tag"
  exit 1
fi
echo "    Latest: $TAG"

URL="https://github.com/$REPO/releases/download/$TAG/$BINARY"
DEST="/usr/local/bin/mcplink"

echo "==> Downloading $BINARY..."
curl -fsSLo /tmp/mcplink "$URL"
chmod +x /tmp/mcplink

echo "==> Installing to $DEST..."
if [ -w /usr/local/bin ]; then
  mv /tmp/mcplink "$DEST"
else
  sudo mv /tmp/mcplink "$DEST"
fi

echo "==> Done! Running first-time setup (requires sudo for service install)..."
sudo mcplink
