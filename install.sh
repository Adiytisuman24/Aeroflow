#!/bin/bash
# AeroFlow Installation Script (Linux/macOS)
# Usage: curl -fsSL https://get.aeroflow.dev/install.sh | sh

set -e

INSTALL_DIR="/usr/local/bin"
AEROFLOW_VERSION="1.0.0"

echo "🌀 AeroFlow Installer v${AEROFLOW_VERSION}"
echo ""

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)  PLATFORM="linux" ;;
  Darwin) PLATFORM="darwin" ;;
  *)      echo "❌ Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64)  ARCH="x86_64" ;;
  aarch64) ARCH="aarch64" ;;
  arm64)   ARCH="aarch64" ;;
  *)       echo "❌ Unsupported architecture: $ARCH"; exit 1 ;;
esac

BINARY_NAME="aeroflow-${PLATFORM}-${ARCH}"
DOWNLOAD_URL="https://github.com/aeroflow/aeroflow/releases/download/v${AEROFLOW_VERSION}/${BINARY_NAME}"

echo "📦 Detected: ${PLATFORM} (${ARCH})"
echo "📥 Downloading AeroFlow..."

# Download binary
if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$DOWNLOAD_URL" -o /tmp/aeroflow
elif command -v wget >/dev/null 2>&1; then
  wget -q "$DOWNLOAD_URL" -O /tmp/aeroflow
else
  echo "❌ Neither curl nor wget found. Please install one."
  exit 1
fi

# Make executable
chmod +x /tmp/aeroflow

# Install to system path
echo "🔧 Installing to ${INSTALL_DIR}..."
if [ -w "$INSTALL_DIR" ]; then
  mv /tmp/aeroflow "$INSTALL_DIR/aeroflow"
else
  sudo mv /tmp/aeroflow "$INSTALL_DIR/aeroflow"
fi

# Verify installation
if command -v aeroflow >/dev/null 2>&1; then
  echo ""
  echo "✅ AeroFlow installed successfully!"
  echo ""
  aeroflow --version
  echo ""
  echo "📚 Next steps:"
  echo "  aeroflow init myapp"
  echo "  cd myapp"
  echo "  aeroflow dev"
  echo ""
  echo "📖 Documentation: https://docs.aeroflow.dev"
else
  echo "❌ Installation failed. Please report this issue."
  exit 1
fi
