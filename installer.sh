#!/usr/bin/env bash
set -euo pipefail

# git-mit installer script for Linux/macOS

# Detect architecture and OS
if [[ "$(uname -s)" == "Darwin" ]]; then
    if [[ "$(uname -m)" == "arm64" ]]; then
        ARCH="aarch64-apple-darwin"
    else
        ARCH="x86_64-apple-darwin"
    fi
elif [[ "$(uname -s)" == "Linux" ]]; then
    ARCH="x86_64-unknown-linux-gnu"
fi

# Create temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR" || exit 1

# Define binaries to download
BINARIES=("git-mit" "git-mit-config" "git-mit-install" "git-mit-relates-to" "mit-commit-msg" "mit-pre-commit" "mit-prepare-commit-msg")

# Download and verify each binary
for binary in "${BINARIES[@]}"; do
    echo "📥 Downloading $binary..."
    curl -sL "https://github.com/PurpleBooth/git-mit/releases/latest/download/${binary}${ARCH}" -o "$binary"
    curl -sL "https://github.com/PurpleBooth/git-mit/releases/latest/download/${binary}${ARCH}.sha256" -o "${binary}.sha256"

    # Verify SHA256
    if sha256sum -c "${binary}.sha256"; then
        echo "✅ Verified $binary"
        chmod +x "$binary"
        sudo mv "$binary" "/usr/local/bin/"
    else
        echo "❌ Verification failed for $binary"
        exit 1
    fi
done

# Cleanup
cd - > /dev/null || exit 1
rm -rf "$TMP_DIR"

echo "🎉 Installation complete! Run 'git mit-install' to set up your repository."
